//! Endpoints REST para a Comunicação de Séries à AT (`SeriesWS`). Cada
//! handler:
//! 1. Valida que o `at_series` está configurado (senão 503).
//! 2. Constrói o `ClientConfig` a partir da config (lê PEM do disco).
//! 3. Invoca o crate `at-series` que faz a chamada SOAP autenticada.
//! 4. Persiste o resultado relevante (e.g., grava `codValidacaoSerie` em
//!    `atcud` no caso do `registar`).

use std::sync::Arc;

use ::at_series::{
    AnularSerieRequest, ClientConfig, ConsultarSeriesRequest, FinalizarSerieRequest,
    RegistarSerieRequest, SeriesClient, SeriesInfo, TipoSerie,
};
use axum::{extract::State, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use server::{AppState, AtSeriesConfig};

use crate::error::ApiError;
use crate::ApiResult;

/// Constrói o `SeriesClient` a partir da config corrente. Lê o PEM do disco
/// a cada chamada — barato e garante hot-reload se o operador substituir a
/// chave em produção sem reiniciar.
fn build_client(cfg: &AtSeriesConfig) -> Result<SeriesClient, ApiError> {
    let public_key_pem = std::fs::read_to_string(&cfg.public_key_path).map_err(|e| {
        ApiError::Internal(format!(
            "AT public key {}: {e}",
            cfg.public_key_path.display()
        ))
    })?;
    let client_identity_pkcs12 = std::fs::read(&cfg.client_pfx_path).map_err(|e| {
        ApiError::Internal(format!(
            "AT client PFX {}: {e}",
            cfg.client_pfx_path.display()
        ))
    })?;
    SeriesClient::new(ClientConfig {
        endpoint: cfg.endpoint.clone(),
        username: cfg.username.clone(),
        password: cfg.password.clone(),
        public_key_pem,
        client_identity_pkcs12,
        client_identity_password: cfg.client_pfx_password.clone(),
        timeout: None,
    })
    .map_err(|e| ApiError::Internal(format!("AT client: {e}")))
}

fn at_cfg(state: &Arc<AppState>) -> ApiResult<&AtSeriesConfig> {
    state
        .config
        .at_series
        .as_ref()
        .ok_or_else(|| ApiError::BadRequest("AT SeriesWS não configurado".into()))
}

#[derive(Deserialize)]
pub struct RegistarSerieBody {
    /// Identificador da série (e.g., "A"). Vai como `serie` na chamada AT.
    pub serie: String,
    /// `N` (Normal) ou `T` (Substituição). Default: Normal.
    #[serde(default)]
    pub tipo_serie: Option<String>,
    /// Classe do documento (e.g., "SI" para facturas).
    pub classe_doc: String,
    /// Tipo do documento (e.g., "FS", "FT", "FR").
    pub tipo_doc: String,
    pub num_inicial_seq: u64,
    pub data_inicio_prev_utiliz: NaiveDate,
    pub meio_processamento: String,
    /// Override opcional do número de certificado (default: o da config).
    pub num_cert_sw_fatur: Option<u32>,
}

#[derive(Serialize)]
pub struct SerieResponse {
    pub info: SeriesInfo,
    /// Quando o backend conseguiu reflectir o resultado na BD (e.g., gravar
    /// o ATCUD após `registar`). False = a chamada AT correu bem mas a BD
    /// local não pôde ser actualizada (e.g., tabela `atcud` em falta).
    pub persisted: bool,
}

pub async fn registar(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegistarSerieBody>,
) -> ApiResult<Json<SerieResponse>> {
    let cfg = at_cfg(&state)?;
    let tipo_serie = match body.tipo_serie.as_deref() {
        Some("T") | Some("Substituicao") => TipoSerie::Substituicao,
        _ => TipoSerie::Normal,
    };
    let req = RegistarSerieRequest {
        serie: body.serie.clone(),
        tipo_serie,
        classe_doc: body.classe_doc.clone(),
        tipo_doc: body.tipo_doc.clone(),
        num_inicial_seq: body.num_inicial_seq,
        data_inicio_prev_utiliz: body.data_inicio_prev_utiliz,
        num_cert_sw_fatur: body.num_cert_sw_fatur.unwrap_or(cfg.num_cert_sw_fatur),
        meio_processamento: body.meio_processamento.clone(),
    };
    let client = build_client(cfg)?;
    let info = client.registar_serie(&req).await.map_err(map_at_error)?;

    // Grava o codValidacaoSerie na tabela atcud para que `allocate_series_number`
    // o use ao emitir documentos. A chave é (tipo_doc, serie, year-da-data).
    let year = body.data_inicio_prev_utiliz.year();
    let persisted = storage::upsert_atcud(
        state.db.pool(),
        &body.tipo_doc,
        &body.serie,
        year,
        &info.cod_validacao_serie,
        body.data_inicio_prev_utiliz,
    )
    .await
    .map(|_| true)
    .unwrap_or(false);

    Ok(Json(SerieResponse { info, persisted }))
}

#[derive(Deserialize, Default)]
pub struct ConsultarSeriesBody {
    pub serie: Option<String>,
    pub tipo_serie: Option<String>,
    pub classe_doc: Option<String>,
    pub tipo_doc: Option<String>,
    pub cod_validacao_serie: Option<String>,
    pub data_registo_de: Option<NaiveDate>,
    pub data_registo_ate: Option<NaiveDate>,
    pub estado: Option<String>,
    pub meio_processamento: Option<String>,
}

#[derive(Serialize)]
pub struct ConsultarResponse {
    pub items: Vec<SeriesInfo>,
}

pub async fn consultar(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ConsultarSeriesBody>,
) -> ApiResult<Json<ConsultarResponse>> {
    let cfg = at_cfg(&state)?;
    let req = ConsultarSeriesRequest {
        serie: body.serie,
        tipo_serie: body.tipo_serie.and_then(|s| match s.as_str() {
            "T" | "Substituicao" => Some(TipoSerie::Substituicao),
            "N" | "Normal" => Some(TipoSerie::Normal),
            _ => None,
        }),
        classe_doc: body.classe_doc,
        tipo_doc: body.tipo_doc,
        cod_validacao_serie: body.cod_validacao_serie,
        data_registo_de: body.data_registo_de,
        data_registo_ate: body.data_registo_ate,
        estado: body.estado,
        meio_processamento: body.meio_processamento,
    };
    let client = build_client(cfg)?;
    let items = client.consultar_series(&req).await.map_err(map_at_error)?;
    Ok(Json(ConsultarResponse { items }))
}

#[derive(Deserialize)]
pub struct FinalizarSerieBody {
    pub serie: String,
    pub classe_doc: String,
    pub tipo_doc: String,
    pub cod_validacao_serie: String,
    pub seq_ultimo_doc_emitido: u64,
    pub justificacao: Option<String>,
    /// Ano da série a finalizar — usado para localizar o ATCUD local a
    /// desactivar. Default: ano corrente.
    pub year: Option<i32>,
}

pub async fn finalizar(
    State(state): State<Arc<AppState>>,
    Json(body): Json<FinalizarSerieBody>,
) -> ApiResult<Json<SerieResponse>> {
    let cfg = at_cfg(&state)?;
    let req = FinalizarSerieRequest {
        serie: body.serie.clone(),
        classe_doc: body.classe_doc,
        tipo_doc: body.tipo_doc.clone(),
        cod_validacao_serie: body.cod_validacao_serie,
        seq_ultimo_doc_emitido: body.seq_ultimo_doc_emitido,
        justificacao: body.justificacao,
    };
    let client = build_client(cfg)?;
    let info = client.finalizar_serie(&req).await.map_err(map_at_error)?;

    let year = body.year.unwrap_or_else(|| chrono::Utc::now().year());
    let persisted = storage::deactivate_atcud(state.db.pool(), &body.tipo_doc, &body.serie, year)
        .await
        .is_ok();
    Ok(Json(SerieResponse { info, persisted }))
}

#[derive(Deserialize)]
pub struct AnularSerieBody {
    pub serie: String,
    pub classe_doc: String,
    pub tipo_doc: String,
    pub cod_validacao_serie: String,
    pub motivo: String,
    /// Confirmação obrigatória (WSDL) — sem isto a AT recusa.
    pub declaracao_nao_emissao: bool,
    pub year: Option<i32>,
}

pub async fn anular(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AnularSerieBody>,
) -> ApiResult<Json<SerieResponse>> {
    let cfg = at_cfg(&state)?;
    let req = AnularSerieRequest {
        serie: body.serie.clone(),
        classe_doc: body.classe_doc,
        tipo_doc: body.tipo_doc.clone(),
        cod_validacao_serie: body.cod_validacao_serie,
        motivo: body.motivo,
        declaracao_nao_emissao: body.declaracao_nao_emissao,
    };
    let client = build_client(cfg)?;
    let info = client.anular_serie(&req).await.map_err(map_at_error)?;

    let year = body.year.unwrap_or_else(|| chrono::Utc::now().year());
    let persisted = storage::deactivate_atcud(state.db.pool(), &body.tipo_doc, &body.serie, year)
        .await
        .is_ok();
    Ok(Json(SerieResponse { info, persisted }))
}

fn map_at_error(e: ::at_series::AtError) -> ApiError {
    use ::at_series::AtError;
    match e {
        AtError::AtFault { code, msg } => {
            ApiError::BadRequest(format!("AT [{code}]: {msg}"))
        }
        AtError::Config(s) => ApiError::Internal(format!("config AT: {s}")),
        AtError::Security(s) => ApiError::Internal(format!("WS-Security: {s}")),
        AtError::Parse(s) => ApiError::Internal(format!("parse resposta AT: {s}")),
        AtError::Http(e) => ApiError::Internal(format!("HTTP AT: {e}")),
    }
}

// Imports adicionais usados acima.
use chrono::Datelike;
