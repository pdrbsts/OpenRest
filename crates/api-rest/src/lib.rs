use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use server::{AppState, SystemEvent};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use domain::{
    Anulacao, Article, Atcud, Cancelamento, Customer, DeliveryEstado, Dispositivo, Document,
    DocumentDetail, DocumentSeries, DocumentTemplate, Employee, Entregador, Family,
    ImpressoraZonaLocal, Local, MesaEstado, Payment, PaymentMethod, PedidoDelivery, SessaoEmpregado,
    Table, TipoPreco, Transferencia, Zona, ZonaImpressao,
};

mod at_series;
mod error;
use error::ApiError;

pub type ApiResult<T> = Result<T, ApiError>;

/// Tri-state for PATCH-like JSON: `Missing` (key not present), `Set(value)`.
/// `Set(None)` represents an explicit JSON `null`.
#[derive(Debug)]
pub enum OptionalField<T> {
    Missing,
    Set(T),
}

impl<T> Default for OptionalField<T> {
    fn default() -> Self {
        OptionalField::Missing
    }
}

impl<T> OptionalField<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            OptionalField::Missing => None,
            OptionalField::Set(v) => Some(v),
        }
    }
}

fn deserialize_optional_field<'de, D, T>(
    deserializer: D,
) -> Result<OptionalField<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Ok(OptionalField::Set(T::deserialize(deserializer)?))
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health))
        .route("/api/system/current-day", get(get_current_day))
        .route("/api/catalog", get(get_catalog))
        .route("/api/locais", get(get_locais).post(create_local))
        .route(
            "/api/locais/:id",
            get(get_local).put(update_local).delete(delete_local),
        )
        .route(
            "/api/locais/:id/tables",
            get(get_local_tables).post(create_local_table),
        )
        .route("/api/tables", get(get_tables))
        .route("/api/tables/:id", put(update_table).delete(delete_table_route))
        .route("/api/tables/:id/open", post(open_table))
        .route("/api/tables/:id/document", get(get_table_document))
        .route("/api/employees", get(get_employees))
        .route("/api/sessoes", get(list_sessoes_handler).post(open_sessao_handler))
        .route("/api/sessoes/:id/fechar", post(close_sessao_handler))
        .route(
            "/api/employees/:id/sessao-aberta",
            get(get_open_sessao_handler),
        )
        .route("/api/payment-methods", get(get_payment_methods))
        .route("/api/series", get(get_series))
        .route("/api/atcuds", get(get_atcuds))
        .route("/api/customers", get(get_customers).post(create_customer))
        .route("/api/customers/search", get(search_customers))
        .route("/api/customers/:id", get(get_customer).put(update_customer))
        .route("/api/customers/:id/forget", post(forget_customer))
        .route(
            "/api/locais/:id/start-document",
            post(start_local_document),
        )
        .route("/api/locais/:id/consumo", post(open_consumo_proprio))
        .route("/api/documents/:id/context", post(set_document_context))
        .route("/api/deliveries", get(get_active_deliveries))
        .route("/api/deliveries/:id/state", post(update_delivery_state))
        .route("/api/tipos-preco", get(get_tipos_preco))
        .route("/api/zonas", get(get_zonas).post(create_zona))
        .route("/api/zonas/:id", put(update_zona).delete(delete_zona))
        .route("/api/entregadores", get(get_entregadores).post(create_entregador))
        .route("/api/entregadores/:id", put(update_entregador).delete(delete_entregador))
        .route("/api/zonas-impressao", get(get_zonas_impressao).post(create_zona_impressao))
        .route(
            "/api/zonas-impressao/:id",
            put(update_zona_impressao).delete(delete_zona_impressao),
        )
        .route("/api/dispositivos", get(get_dispositivos).post(create_dispositivo))
        .route("/api/dispositivos/:id/status", get(get_dispositivo_status))
        .route("/api/dispositivos/:id/test", post(test_dispositivo))
        .route(
            "/api/dispositivos/:id",
            put(update_dispositivo).delete(delete_dispositivo),
        )
        .route("/api/print-mappings", get(get_print_mappings).post(create_print_mapping))
        .route("/api/print-mappings/:id", axum::routing::delete(delete_print_mapping))
        .route("/api/documents/:id/pedir", post(pedir_document))
        .route("/api/documents/:id", get(get_document))
        .route("/api/documents/:id/lines", post(add_line))
        .route(
            "/api/documents/:id/lines/:line_id",
            axum::routing::delete(cancel_line),
        )
        .route(
            "/api/documents/:id/lines/:line_id/anular",
            post(anular_line),
        )
        .route("/api/anulacoes", get(get_anulacoes))
        .route("/api/cancelamentos", get(get_cancelamentos))
        .route("/api/transferencias", get(get_transferencias))
        .route("/api/documents/:id/transfer", post(transfer_document))
        .route("/api/documents/:id/close", post(close_document))
        .route("/api/documents/:id/partial-close", post(partial_close_document))
        .route("/api/documents/:id/split", post(split_document_handler))
        .route("/api/documents/:id/split/auto-plan", get(auto_split_plan))
        .route("/api/documents/:id/print", post(print_document))
        .route("/api/document-templates", get(get_document_templates))
        .route(
            "/api/document-templates/:tipo",
            get(get_document_template_handler).put(update_document_template_handler),
        )
        .route("/api/at-series/registar", post(at_series::registar))
        .route("/api/at-series/consultar", post(at_series::consultar))
        .route("/api/at-series/finalizar", post(at_series::finalizar))
        .route("/api/at-series/anular", post(at_series::anular))
        .with_state(state)
        .layer(cors)
}

async fn health() -> &'static str {
    "ok"
}

#[derive(Serialize)]
pub struct CurrentDayResponse {
    pub data_dia: chrono::NaiveDate,
    pub server_now: chrono::DateTime<Utc>,
    pub cutoff_minutes: u32,
    pub tz_offset_minutes: i32,
}

async fn get_current_day(State(state): State<Arc<AppState>>) -> Json<CurrentDayResponse> {
    let now = Utc::now();
    Json(CurrentDayResponse {
        data_dia: state.config.business_day(now),
        server_now: now,
        cutoff_minutes: state.config.business_day_cutoff_minutes,
        tz_offset_minutes: state.config.business_day_tz_offset_minutes,
    })
}

#[derive(Serialize)]
pub struct CatalogResponse {
    pub families: Vec<Family>,
    pub articles: Vec<Article>,
}

#[derive(Serialize)]
pub struct DocumentResponse {
    pub document: Document,
    pub lines: Vec<DocumentDetail>,
    pub payments: Vec<Payment>,
}

async fn build_doc_response(
    pool: &storage::SqlitePool,
    document: Document,
) -> Result<DocumentResponse, ApiError> {
    let lines = storage::list_document_details(pool, document.id).await?;
    let payments = storage::list_document_payments(pool, document.id).await?;
    Ok(DocumentResponse { document, lines, payments })
}

async fn get_catalog(State(state): State<Arc<AppState>>) -> ApiResult<Json<CatalogResponse>> {
    let pool = state.db.pool();
    Ok(Json(CatalogResponse {
        families: storage::list_families(pool).await?,
        articles: storage::list_articles(pool).await?,
    }))
}

#[derive(Serialize)]
pub struct TableWithEstado {
    #[serde(flatten)]
    pub table: Table,
    pub estado: MesaEstado,
}

async fn assemble_tables(
    pool: &storage::SqlitePool,
    tables: Vec<Table>,
) -> Result<Vec<TableWithEstado>, ApiError> {
    let estados = storage::list_mesa_estados(pool).await?;
    let by_id: std::collections::HashMap<Uuid, MesaEstado> =
        estados.into_iter().map(|e| (e.mesa_id, e)).collect();
    Ok(tables
        .into_iter()
        .map(|t| {
            let estado = by_id.get(&t.id).cloned().unwrap_or(MesaEstado {
                mesa_id: t.id,
                estado: domain::MesaEstadoKind::Livre,
                bloqueada_por_posto_id: None,
                bloqueada_motivo: None,
                cliente_associado_id: None,
                numero_pessoas: None,
                empregado_actual_id: None,
                aberta_em: None,
                subtotal_actual: 0,
                reservada_ate: None,
                reserva_pessoas: None,
                reserva_cliente_id: None,
                reserva_observacoes: None,
            });
            TableWithEstado { table: t, estado }
        })
        .collect())
}

async fn get_tables(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<TableWithEstado>>> {
    let tables = storage::list_tables(state.db.pool()).await?;
    Ok(Json(assemble_tables(state.db.pool(), tables).await?))
}

async fn get_locais(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Local>>> {
    Ok(Json(storage::list_locais(state.db.pool()).await?))
}

async fn get_local(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Local>> {
    Ok(Json(storage::get_local(state.db.pool(), id).await?))
}

async fn get_local_tables(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Vec<TableWithEstado>>> {
    let tables = storage::list_tables_by_local(state.db.pool(), id).await?;
    Ok(Json(assemble_tables(state.db.pool(), tables).await?))
}

#[derive(Deserialize)]
pub struct CreateLocalRequest {
    pub designacao: String,
    #[serde(default = "default_local_kind")]
    pub tipo: String,
    pub nome_generico_mesa: Option<String>,
    #[serde(default)]
    pub usa_desenho_mesas: bool,
    pub imagem: Option<String>,
    pub largura: Option<i32>,
    pub altura: Option<i32>,
}

fn default_local_kind() -> String {
    "normal".to_string()
}

async fn create_local(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateLocalRequest>,
) -> ApiResult<(StatusCode, Json<Local>)> {
    let tipo = domain::LocalKind::parse(&req.tipo)
        .ok_or_else(|| ApiError::BadRequest(format!("tipo inválido: {}", req.tipo)))?;
    let new = storage::NewLocal {
        designacao: req.designacao,
        tipo,
        nome_generico_mesa: req.nome_generico_mesa,
        usa_desenho_mesas: req.usa_desenho_mesas,
        imagem: req.imagem,
        largura: req.largura,
        altura: req.altura,
    };
    let local = storage::create_local(state.db.pool(), new).await?;
    Ok((StatusCode::CREATED, Json(local)))
}

#[derive(Deserialize, Default)]
pub struct UpdateLocalRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub designacao: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub tipo: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub nome_generico_mesa: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub usa_desenho_mesas: OptionalField<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub imagem: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub largura: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub altura: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub mesas_definicao: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub permite_zero_pessoas: OptionalField<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub permite_mesas_abertas_fim_do_dia: OptionalField<bool>,
}

async fn update_local(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateLocalRequest>,
) -> ApiResult<Json<Local>> {
    let tipo = match req.tipo.into_option() {
        Some(s) => Some(
            domain::LocalKind::parse(&s)
                .ok_or_else(|| ApiError::BadRequest(format!("tipo inválido: {}", s)))?,
        ),
        None => None,
    };
    let upd = storage::LocalUpdate {
        designacao: req.designacao.into_option(),
        tipo,
        nome_generico_mesa: req.nome_generico_mesa.into_option(),
        usa_desenho_mesas: req.usa_desenho_mesas.into_option(),
        imagem: req.imagem.into_option(),
        largura: req.largura.into_option(),
        altura: req.altura.into_option(),
        mesas_definicao: req.mesas_definicao.into_option(),
        permite_zero_pessoas: req.permite_zero_pessoas.into_option(),
        permite_mesas_abertas_fim_do_dia: req.permite_mesas_abertas_fim_do_dia.into_option(),
    };
    let local = storage::update_local(state.db.pool(), id, upd).await?;
    Ok(Json(local))
}

async fn delete_local(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_local(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct CreateTableRequest {
    pub code: i32,
    pub name: Option<String>,
    pub posx: Option<i32>,
    pub posy: Option<i32>,
    pub altura: Option<i32>,
    pub largura: Option<i32>,
    pub imagem: Option<String>,
}

async fn create_local_table(
    State(state): State<Arc<AppState>>,
    Path(local_id): Path<Uuid>,
    Json(req): Json<CreateTableRequest>,
) -> ApiResult<(StatusCode, Json<Table>)> {
    storage::get_local(state.db.pool(), local_id).await?;
    let new = storage::NewTable {
        local_id: Some(local_id),
        code: req.code,
        name: req.name,
        posx: req.posx,
        posy: req.posy,
        altura: req.altura,
        largura: req.largura,
        imagem: req.imagem,
    };
    let table = storage::create_table(state.db.pool(), new).await?;
    Ok((StatusCode::CREATED, Json(table)))
}

#[derive(Deserialize, Default)]
pub struct UpdateTableRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub local_id: OptionalField<Option<Uuid>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub code: OptionalField<i32>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub name: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub nomeobjecto: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub posx: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub posy: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub imagem: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub fntname: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub fntsize: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub fntcolor: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub fontstyle: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub altura: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub largura: OptionalField<Option<i32>>,
}

async fn update_table(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTableRequest>,
) -> ApiResult<Json<Table>> {
    let upd = storage::TableUpdate {
        local_id: req.local_id.into_option(),
        code: req.code.into_option(),
        name: req.name.into_option(),
        nomeobjecto: req.nomeobjecto.into_option(),
        posx: req.posx.into_option(),
        posy: req.posy.into_option(),
        imagem: req.imagem.into_option(),
        fntname: req.fntname.into_option(),
        fntsize: req.fntsize.into_option(),
        fntcolor: req.fntcolor.into_option(),
        fontstyle: req.fontstyle.into_option(),
        altura: req.altura.into_option(),
        largura: req.largura.into_option(),
    };
    let table = storage::update_table(state.db.pool(), id, upd).await?;
    Ok(Json(table))
}

async fn delete_table_route(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_table(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_employees(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Employee>>> {
    Ok(Json(storage::list_employees(state.db.pool()).await?))
}

// --- Sessões de empregado --------------------------------------------------

#[derive(Deserialize)]
pub struct ListSessoesQuery {
    #[serde(default)]
    pub apenas_abertas: bool,
}

async fn list_sessoes_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(q): axum::extract::Query<ListSessoesQuery>,
) -> ApiResult<Json<Vec<SessaoEmpregado>>> {
    Ok(Json(
        storage::list_sessoes(state.db.pool(), q.apenas_abertas).await?,
    ))
}

async fn get_open_sessao_handler(
    State(state): State<Arc<AppState>>,
    Path(employee_id): Path<Uuid>,
) -> ApiResult<Json<Option<SessaoEmpregado>>> {
    Ok(Json(
        storage::get_open_sessao_for_employee(state.db.pool(), employee_id).await?,
    ))
}

#[derive(Deserialize)]
pub struct OpenSessaoRequest {
    pub empregado_id: Uuid,
    #[serde(default)]
    pub com_bolsa: bool,
    #[serde(default)]
    pub fundo_bolsa: i64,
    pub observacao: Option<String>,
    /// Empregado que está a abrir (e.g. supervisor). Default: o próprio.
    pub aberta_por: Option<Uuid>,
}

async fn open_sessao_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<OpenSessaoRequest>,
) -> ApiResult<(StatusCode, Json<SessaoEmpregado>)> {
    let pool = state.db.pool();
    // Garante que o empregado existe (evita FK pendurada).
    storage::get_employee(pool, req.empregado_id).await?;
    let data_dia = state.config.business_day(Utc::now());
    let sessao = storage::open_sessao(
        pool,
        storage::NewSessao {
            empregado_id: req.empregado_id,
            data_dia,
            com_bolsa: req.com_bolsa,
            fundo_bolsa: req.fundo_bolsa,
            observacao_abertura: req.observacao,
            aberta_por: req.aberta_por.or(Some(req.empregado_id)),
        },
    )
    .await
    .map_err(|e| match &e {
        storage::StorageError::Database(storage::sqlx::Error::Protocol(msg)) => {
            ApiError::BadRequest(msg.clone())
        }
        _ => ApiError::from(e),
    })?;
    Ok((StatusCode::CREATED, Json(sessao)))
}

#[derive(Deserialize, Default)]
pub struct CloseSessaoRequest {
    pub observacao: Option<String>,
    pub fechada_por: Option<Uuid>,
}

async fn close_sessao_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    body: Option<Json<CloseSessaoRequest>>,
) -> ApiResult<Json<SessaoEmpregado>> {
    let req = body.map(|b| b.0).unwrap_or_default();
    let sessao = storage::close_sessao(state.db.pool(), id, req.observacao, req.fechada_por)
        .await
        .map_err(|e| match &e {
            storage::StorageError::Database(storage::sqlx::Error::Protocol(msg)) => {
                ApiError::BadRequest(msg.clone())
            }
            _ => ApiError::from(e),
        })?;
    Ok(Json(sessao))
}

async fn get_payment_methods(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<PaymentMethod>>> {
    Ok(Json(storage::list_payment_methods(state.db.pool()).await?))
}

async fn get_series(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<DocumentSeries>>> {
    Ok(Json(storage::list_series(state.db.pool()).await?))
}

async fn get_atcuds(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Atcud>>> {
    Ok(Json(storage::list_atcuds(state.db.pool()).await?))
}

#[derive(Deserialize, Default)]
pub struct OpenTableRequest {
    pub employee_id: Option<Uuid>,
}

async fn open_table(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    body: Option<Json<OpenTableRequest>>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let employee_id = body.and_then(|b| b.employee_id);
    // Spec §4 (Sessões de Empregado): só se entra numa mesa com sessão aberta.
    require_open_sessao(pool, employee_id).await?;
    let business_date = state.config.business_day(Utc::now());
    let document =
        storage::open_table(pool, id, employee_id, business_date).await?;
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentCreated { document_id: document.id });
    Ok(Json(build_doc_response(state.db.pool(), document).await?))
}

async fn get_table_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Option<DocumentResponse>>> {
    let pool = state.db.pool();
    let Some(document) = storage::get_open_document_for_table(pool, id).await? else {
        return Ok(Json(None));
    };
    Ok(Json(Some(build_doc_response(pool, document).await?)))
}

async fn get_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let document = storage::get_document(pool, id).await?;
    Ok(Json(build_doc_response(pool, document).await?))
}

#[derive(Deserialize)]
pub struct AddLineRequest {
    pub article_id: Uuid,
    pub qty: i32,
}

async fn add_line(
    State(state): State<Arc<AppState>>,
    Path(document_id): Path<Uuid>,
    Json(req): Json<AddLineRequest>,
) -> ApiResult<Json<DocumentResponse>> {
    if req.qty <= 0 {
        return Err(ApiError::BadRequest("qty must be > 0".into()));
    }
    let pool = state.db.pool();
    let document = storage::get_document(pool, document_id).await?;
    if document.is_closed {
        return Err(ApiError::BadRequest("document is closed".into()));
    }
    let article = storage::get_article(pool, req.article_id).await?;
    // Preço escolhido pelo tipo_preco do local
    let unit_price = if let Some(local_id) = document.local_id {
        let local = storage::get_local(pool, local_id).await?;
        storage::price_for_local(pool, &article, &local).await?
    } else {
        article.pvp1
    };
    storage::add_document_line(pool, document_id, article.id, req.qty, unit_price).await?;
    let document = storage::get_document(pool, document_id).await?;
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentLineAdded { document_id });
    Ok(Json(build_doc_response(pool, document).await?))
}

/// Resolve `employee_id` → `NivelAcesso`. Returns 400 if missing, 404 if unknown,
/// 403 if the employee has no nível atribuído.
async fn require_nivel(
    pool: &storage::sqlx::SqlitePool,
    employee_id: Option<Uuid>,
) -> ApiResult<domain::NivelAcesso> {
    let employee_id =
        employee_id.ok_or_else(|| ApiError::BadRequest("employee_id is required".into()))?;
    let employee = storage::get_employee(pool, employee_id).await?;
    let nivel_id = employee
        .nivel_acesso_id
        .ok_or_else(|| ApiError::Forbidden("employee has no nivel_acesso".into()))?;
    Ok(storage::get_nivel_acesso(pool, nivel_id).await?)
}

/// Spec §4: garante que o empregado está identificado e tem sessão aberta.
/// 400 se faltar `employee_id`, 403 se não houver sessão aberta.
async fn require_open_sessao(
    pool: &storage::sqlx::SqlitePool,
    employee_id: Option<Uuid>,
) -> ApiResult<SessaoEmpregado> {
    let employee_id =
        employee_id.ok_or_else(|| ApiError::BadRequest("employee_id is required".into()))?;
    storage::get_open_sessao_for_employee(pool, employee_id)
        .await?
        .ok_or_else(|| {
            ApiError::Forbidden("empregado sem sessão aberta — abra sessão primeiro".into())
        })
}

#[derive(Deserialize, Default)]
pub struct CancelLineRequest {
    pub motivo: Option<String>,
    pub employee_id: Option<Uuid>,
}

async fn cancel_line(
    State(state): State<Arc<AppState>>,
    Path((document_id, line_id)): Path<(Uuid, Uuid)>,
    body: Option<Json<CancelLineRequest>>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let document = storage::get_document(pool, document_id).await?;
    if document.is_closed {
        return Err(ApiError::BadRequest("document already closed".into()));
    }
    let req = body.map(|b| b.0).unwrap_or_default();
    let nivel = require_nivel(pool, req.employee_id).await?;
    if !nivel.cancela_pedidos {
        return Err(ApiError::Forbidden(
            "nível de acesso sem permissão 'pedidos.cancelar'".into(),
        ));
    }
    storage::cancel_document_line(
        pool,
        document_id,
        line_id,
        state.config.registar_cancelamentos,
        req.motivo,
        req.employee_id,
    )
    .await
    .map_err(|e| match &e {
        storage::StorageError::Database(storage::sqlx::Error::Protocol(msg)) => {
            ApiError::BadRequest(msg.clone())
        }
        _ => ApiError::from(e),
    })?;
    let document = storage::get_document(pool, document_id).await?;
    Ok(Json(build_doc_response(pool, document).await?))
}

async fn get_anulacoes(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Anulacao>>> {
    Ok(Json(storage::list_anulacoes(state.db.pool()).await?))
}

async fn get_cancelamentos(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<Cancelamento>>> {
    Ok(Json(storage::list_cancelamentos(state.db.pool()).await?))
}

async fn get_transferencias(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<Transferencia>>> {
    Ok(Json(storage::list_transferencias(state.db.pool()).await?))
}

#[derive(Deserialize)]
pub struct TransferRequest {
    pub target_table_id: Uuid,
    #[serde(default)]
    pub line_ids: Option<Vec<Uuid>>,
    pub employee_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct TransferResponse {
    pub from_document: DocumentResponse,
    pub to_document: DocumentResponse,
    pub transferencias: Vec<Transferencia>,
}

async fn transfer_document(
    State(state): State<Arc<AppState>>,
    Path(document_id): Path<Uuid>,
    Json(req): Json<TransferRequest>,
) -> ApiResult<Json<TransferResponse>> {
    let pool = state.db.pool();
    let from_doc = storage::get_document(pool, document_id).await?;
    if from_doc.is_closed {
        return Err(ApiError::BadRequest("document already closed".into()));
    }
    let nivel = require_nivel(pool, req.employee_id).await?;
    if !nivel.transfere_pedidos {
        return Err(ApiError::Forbidden(
            "nível de acesso sem permissão 'pedidos.transferencias'".into(),
        ));
    }
    if from_doc.subtotal_impresso_em.is_some() && !nivel.transfere_pedidos_com_conta_impressa {
        return Err(ApiError::Forbidden(
            "nível de acesso sem permissão 'pedidos.transferencias.com_conta_impressa'".into(),
        ));
    }

    let line_ids_slice = req.line_ids.as_deref();
    let business_date = state.config.business_day(Utc::now());
    let (to_doc, transferencias) = storage::transfer_document_lines(
        pool,
        document_id,
        req.target_table_id,
        line_ids_slice,
        req.employee_id,
        business_date,
    )
    .await
    .map_err(|e| match &e {
        storage::StorageError::Database(storage::sqlx::Error::Protocol(msg)) => {
            ApiError::BadRequest(msg.clone())
        }
        _ => ApiError::from(e),
    })?;

    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentLineAdded { document_id: to_doc.id });
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentLineAdded { document_id });

    let from_doc = storage::get_document(pool, document_id).await?;
    Ok(Json(TransferResponse {
        from_document: build_doc_response(pool, from_doc).await?,
        to_document: build_doc_response(pool, to_doc).await?,
        transferencias,
    }))
}

#[derive(Deserialize)]
pub struct AnularLineRequest {
    #[serde(default)]
    pub com_desperdicio: bool,
    pub motivo: Option<String>,
    pub employee_id: Option<Uuid>,
}

async fn anular_line(
    State(state): State<Arc<AppState>>,
    Path((document_id, line_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<AnularLineRequest>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let document = storage::get_document(pool, document_id).await?;
    if document.is_closed {
        return Err(ApiError::BadRequest("document already closed".into()));
    }
    let nivel = require_nivel(pool, req.employee_id).await?;
    if !nivel.anula_pedidos {
        return Err(ApiError::Forbidden(
            "nível de acesso sem permissão 'pedidos.anula'".into(),
        ));
    }
    // Spec §10: se a conta (sub-total) já foi impressa, exige permissão extra.
    if document.subtotal_impresso_em.is_some() && !nivel.anula_pedidos_com_conta_impressa {
        return Err(ApiError::Forbidden(
            "nível de acesso sem permissão 'pedidos.anula.com_conta_impressa'".into(),
        ));
    }
    let line = storage::anular_document_line(
        pool,
        document_id,
        line_id,
        req.com_desperdicio,
        req.motivo.clone(),
        req.employee_id,
    )
    .await
    .map_err(|e| match &e {
        storage::StorageError::Database(storage::sqlx::Error::Protocol(msg)) => {
            ApiError::BadRequest(msg.clone())
        }
        _ => ApiError::from(e),
    })?;

    // Imprime ticket de anulação na zona original do artigo (spec §10 "imprime na zona original").
    let article = storage::get_article(pool, line.article_id).await?;
    if let (Some(local_id), Some(zona_id)) = (document.local_id, article.zona_impressao_id) {
        let local = storage::get_local(pool, local_id).await?;
        if let Some(dispositivo) =
            storage::dispositivo_for_zona_local(pool, zona_id, local.id).await?
        {
            let zona = storage::get_zona_impressao(pool, zona_id).await?;
            let table_label = match document.table_id {
                Some(tid) => storage::get_table(pool, tid)
                    .await
                    .ok()
                    .and_then(|t| t.name.or(Some(format!("Mesa {}", t.code))))
                    .unwrap_or_else(|| "Mesa".into()),
                None => "Balcão".into(),
            };
            let ticket = devices::escpos::format_anulacao_ticket(
                &zona.designacao,
                &local.designacao,
                &table_label,
                Utc::now(),
                line.qty,
                &article.name,
                req.com_desperdicio,
                req.motivo.as_deref(),
            );
            enqueue_ticket(&state, &dispositivo, &ticket).await;
        }
    }

    let document = storage::get_document(pool, document_id).await?;
    Ok(Json(build_doc_response(pool, document).await?))
}

#[derive(Deserialize, Default)]
pub struct CloseDocumentRequest {
    /// Atalho mono-método (mantido para compatibilidade): regista um único
    /// rodapé pelo total do documento. Ignorado se `payments` vier preenchido.
    pub payment_method_id: Option<Uuid>,
    /// Rodapés de pagamento (1..N métodos). Soma >= total; o excedente é
    /// gravado como troco no documento.
    #[serde(default)]
    pub payments: Vec<PaymentLineRequest>,
}

#[derive(Deserialize, Debug)]
pub struct PaymentLineRequest {
    pub payment_method_id: Uuid,
    pub amount: i64,
    pub descricao: Option<String>,
}

async fn close_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    body: Option<Json<CloseDocumentRequest>>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let req = body.map(|b| b.0).unwrap_or_default();
    let document = storage::get_document(pool, id).await?;
    let payment_inputs = build_payment_inputs(&req, document.total)?;
    let document = fiscal_close_document(&state, id, &payment_inputs).await?;
    Ok(Json(build_doc_response(pool, document).await?))
}

/// Converte o body do request para uma lista de `PaymentInput` consumível
/// pelo storage. O atalho mono-método é expandido para um único rodapé pelo
/// `document_total`. Validações: amount > 0 em modo multi-método.
fn build_payment_inputs(
    req: &CloseDocumentRequest,
    document_total: i64,
) -> ApiResult<Vec<storage::PaymentInput>> {
    if !req.payments.is_empty() {
        for p in &req.payments {
            if p.amount <= 0 {
                return Err(ApiError::BadRequest(
                    "payment amount must be positive".into(),
                ));
            }
        }
        Ok(req
            .payments
            .iter()
            .map(|p| storage::PaymentInput {
                payment_method_id: p.payment_method_id,
                amount: p.amount,
                descricao: p.descricao.clone(),
            })
            .collect())
    } else if let Some(method_id) = req.payment_method_id {
        Ok(vec![storage::PaymentInput {
            payment_method_id: method_id,
            amount: document_total,
            descricao: None,
        }])
    } else {
        Ok(Vec::new())
    }
}

/// Núcleo do fecho fiscal: aloca série, calcula hash/ATCUD/QR, grava rodapés
/// e marca o documento fechado. Partilhado entre `close_document` e
/// `partial_close_document`.
async fn fiscal_close_document(
    state: &Arc<AppState>,
    document_id: Uuid,
    payment_inputs: &[storage::PaymentInput],
) -> ApiResult<Document> {
    let pool = state.db.pool();
    let document = storage::get_document(pool, document_id).await?;
    if document.is_closed {
        return Err(ApiError::BadRequest("document already closed".into()));
    }
    if document.total <= 0 {
        return Err(ApiError::BadRequest("document has no lines".into()));
    }
    let payments_sum: i64 = payment_inputs.iter().map(|p| p.amount).sum();
    if !payment_inputs.is_empty() && payments_sum < document.total {
        return Err(ApiError::BadRequest(format!(
            "payments sum ({}) below document total ({})",
            payments_sum, document.total
        )));
    }

    let lines = storage::list_document_details(pool, document.id).await?;
    let mut articles = Vec::with_capacity(lines.len());
    for l in &lines {
        articles.push(storage::get_article(pool, l.article_id).await?);
    }
    let breakdown = compute_vat_breakdown(&lines, &articles);
    let issued_at = Utc::now();
    let year = issued_at.year();

    let mut tx = pool.begin().await?;
    let (series, atcud_entry, number) =
        storage::allocate_series_number(&mut tx, "FS", year).await?;
    let document_identifier = format!("{} {}/{}", series.document_type, series.prefix, number);
    let atcud = fiscal::atcud(&atcud_entry.atcud, number);

    let previous_hash = storage::last_hash_for_series(&mut tx, series.id).await?.unwrap_or_default();
    let payload = fiscal::signing_payload(
        issued_at,
        issued_at,
        &document_identifier,
        document.total,
        &previous_hash,
    );
    let (hash, hash_short) = fiscal::sign(&state.config.signing_key, &payload);

    let vat_lines: Vec<(fiscal::VatRate, i64, i64)> = breakdown
        .iter()
        .map(|b| (fiscal::VatRate::from_basis_points(b.rate_bp), b.base, b.vat))
        .collect();
    let total_vat: i64 = breakdown.iter().map(|b| b.vat).sum();
    let qr_payload = fiscal::qr_payload(&fiscal::QrInputs {
        emitter_nif: &state.config.company.nif,
        customer_nif: "999999990",
        country: &state.config.company.country,
        document_type: "FS",
        document_status: "N",
        document_date: issued_at,
        document_identifier: &document_identifier,
        atcud: &atcud,
        tax_country: &state.config.company.country,
        vat_breakdown: &vat_lines,
        total_vat_cents: total_vat,
        total_with_vat_cents: document.total,
        hash_short: &hash_short,
        software_certificate: &state.config.company.software_certificate,
    });

    storage::finalize_document_fiscal(
        &mut tx,
        document.id,
        series.id,
        &series.document_type,
        number,
        &atcud,
        &hash,
        &hash_short,
        &previous_hash,
        issued_at,
        &qr_payload,
    )
    .await?;

    if !payment_inputs.is_empty() {
        storage::record_payments_bulk_tx(&mut tx, document.id, document.total, payment_inputs)
            .await?;
    }

    tx.commit().await?;

    let document = storage::get_document(pool, document.id).await?;
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentClosed { document_id: document.id });
    Ok(document)
}

#[derive(Deserialize)]
pub struct PartialCloseRequest {
    /// Linhas do pai a transferir para o filho. Têm de estar pedidas e não
    /// anuladas. O cliente envia exactamente as linhas seleccionadas no UI.
    pub line_ids: Vec<Uuid>,
    /// Rodapés de pagamento do filho (1..N métodos) — mesma estrutura do
    /// endpoint principal de fecho.
    #[serde(default)]
    pub payments: Vec<PaymentLineRequest>,
    /// Atalho mono-método. Ignorado se `payments` vier preenchido.
    pub payment_method_id: Option<Uuid>,
}

/// Pagamento parcial: move linhas do pai para um filho recém-criado e fecha
/// o filho fiscalmente. O pai mantém-se aberto com as linhas remanescentes
/// (mesa segue ocupada). Resposta: o `DocumentResponse` do filho fechado.
async fn partial_close_document(
    State(state): State<Arc<AppState>>,
    Path(parent_id): Path<Uuid>,
    Json(req): Json<PartialCloseRequest>,
) -> ApiResult<Json<DocumentResponse>> {
    if req.line_ids.is_empty() {
        return Err(ApiError::BadRequest(
            "partial close requires at least one line".into(),
        ));
    }
    let pool = state.db.pool();

    let child = storage::move_lines_to_new_document(pool, parent_id, &req.line_ids)
        .await
        .map_err(ApiError::from)?;

    let close_req = CloseDocumentRequest {
        payment_method_id: req.payment_method_id,
        payments: req.payments,
    };
    let payment_inputs = build_payment_inputs(&close_req, child.total)?;
    let closed_child = fiscal_close_document(&state, child.id, &payment_inputs).await?;
    Ok(Json(build_doc_response(pool, closed_child).await?))
}

#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum SplitRequest {
    /// Modo Linhas (existing): cada linha vai inteira para uma conta. As
    /// contas têm o total da soma das suas linhas (não necessariamente igual).
    Lines { assignments: Vec<SplitAssignmentRequest> },
    /// Modo Quantidades: cada linha elegível é dividida fraccionariamente em
    /// `num_accounts` partes. Todas as contas ficam exactamente com o mesmo
    /// total (o cêntimo residual é absorvido pelo pai).
    Quantidades { num_accounts: usize },
    /// Modo Encaixar: o operador atribui linhas a contas primárias; o sistema
    /// gera linhas de compensação para igualar totais. Cada conta fica com
    /// `total_elegível / N`.
    Encaixar { assignments: Vec<SplitAssignmentRequest> },
}

#[derive(Deserialize)]
pub struct SplitAssignmentRequest {
    pub line_ids: Vec<Uuid>,
}

#[derive(Serialize)]
pub struct SplitResponse {
    pub children: Vec<DocumentResponse>,
}

/// Divide um documento em N filhos. O `mode` selecciona a estratégia. Cada
/// filho fica aberto, pronto a ser fechado individualmente pelo endpoint
/// `close`. O pai fica `is_closed=true` sem dados fiscais quando ficar sem
/// linhas elegíveis; a mesa é libertada nesse momento.
async fn split_document_handler(
    State(state): State<Arc<AppState>>,
    Path(parent_id): Path<Uuid>,
    Json(req): Json<SplitRequest>,
) -> ApiResult<Json<SplitResponse>> {
    let pool = state.db.pool();
    let children = match req {
        SplitRequest::Lines { assignments } => {
            if assignments.is_empty() {
                return Err(ApiError::BadRequest(
                    "split requires at least one account".into(),
                ));
            }
            let assignments: Vec<storage::SplitAssignment> = assignments
                .into_iter()
                .map(|a| storage::SplitAssignment { line_ids: a.line_ids })
                .collect();
            storage::split_document(pool, parent_id, &assignments)
                .await
                .map_err(ApiError::from)?
        }
        SplitRequest::Quantidades { num_accounts } => {
            storage::split_document_quantidades(pool, parent_id, num_accounts)
                .await
                .map_err(ApiError::from)?
        }
        SplitRequest::Encaixar { assignments } => {
            if assignments.len() < 2 {
                return Err(ApiError::BadRequest(
                    "encaixar requires at least 2 accounts".into(),
                ));
            }
            let assignments: Vec<storage::SplitAssignment> = assignments
                .into_iter()
                .map(|a| storage::SplitAssignment { line_ids: a.line_ids })
                .collect();
            storage::split_document_encaixar(pool, parent_id, &assignments)
                .await
                .map_err(ApiError::from)?
        }
    };
    let mut out = Vec::with_capacity(children.len());
    for c in children {
        out.push(build_doc_response(pool, c).await?);
    }
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentClosed { document_id: parent_id });
    Ok(Json(SplitResponse { children: out }))
}

#[derive(Deserialize)]
pub struct AutoPlanQuery {
    pub num_accounts: usize,
}

#[derive(Serialize)]
pub struct AutoPlanResponse {
    pub assignments: Vec<AutoPlanAccount>,
}

#[derive(Serialize)]
pub struct AutoPlanAccount {
    pub line_ids: Vec<Uuid>,
    pub total: i64,
}

/// Sugestão de divisão automática (greedy LPT) que a UI pode mostrar antes
/// do utilizador confirmar. Não muta a BD. A UI pode aplicar tal-e-qual ou
/// permitir ajustes manuais antes de chamar `POST split`.
async fn auto_split_plan(
    State(state): State<Arc<AppState>>,
    Path(parent_id): Path<Uuid>,
    axum::extract::Query(q): axum::extract::Query<AutoPlanQuery>,
) -> ApiResult<Json<AutoPlanResponse>> {
    if q.num_accounts == 0 {
        return Err(ApiError::BadRequest("num_accounts must be > 0".into()));
    }
    let pool = state.db.pool();
    let lines = storage::list_document_details(pool, parent_id).await?;
    let plan = storage::plan_auto_split(&lines, q.num_accounts);
    let mut accounts = Vec::with_capacity(plan.len());
    for assignment in plan {
        let total: i64 = lines
            .iter()
            .filter(|l| assignment.line_ids.contains(&l.id))
            .map(|l| l.total)
            .sum();
        accounts.push(AutoPlanAccount {
            line_ids: assignment.line_ids,
            total,
        });
    }
    Ok(Json(AutoPlanResponse { assignments: accounts }))
}

struct VatBucket {
    rate_bp: i32,
    base: i64,
    vat: i64,
}

fn compute_vat_breakdown(
    lines: &[DocumentDetail],
    articles: &[Article],
) -> Vec<VatBucket> {
    use std::collections::BTreeMap;
    let mut buckets: BTreeMap<i32, (i64, i64)> = BTreeMap::new();
    for (l, a) in lines.iter().zip(articles.iter()) {
        let entry = buckets.entry(a.vat_rate).or_insert((0, 0));
        // Prices stored include VAT (gross). Base = gross / (1 + rate).
        let rate = a.vat_rate as i64;
        let denom = 10_000 + rate;
        let base = (l.total * 10_000) / denom;
        let vat = l.total - base;
        entry.0 += base;
        entry.1 += vat;
    }
    buckets
        .into_iter()
        .map(|(rate_bp, (base, vat))| VatBucket { rate_bp, base, vat })
        .collect()
}

async fn get_customers(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Customer>>> {
    Ok(Json(storage::list_customers(state.db.pool()).await?))
}

async fn get_customer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Customer>> {
    Ok(Json(storage::get_customer(state.db.pool(), id).await?))
}

#[derive(Deserialize)]
pub struct CustomerSearchQuery {
    pub phone: Option<String>,
    pub name: Option<String>,
}

async fn search_customers(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(q): axum::extract::Query<CustomerSearchQuery>,
) -> ApiResult<Json<Vec<Customer>>> {
    Ok(Json(
        storage::search_customers(state.db.pool(), q.phone.as_deref(), q.name.as_deref())
            .await?,
    ))
}

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub nome: String,
    pub nif: Option<String>,
    pub pais: Option<String>,
    pub telefone: Option<String>,
    pub morada: Option<String>,
    pub cod_postal: Option<String>,
    pub localidade: Option<String>,
    pub email: Option<String>,
    pub observacoes: Option<String>,
    pub zona_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct CustomerResponse {
    #[serde(flatten)]
    pub customer: Customer,
    /// Spec §201: avisamos mas não impedimos.
    pub nif_warning: Option<String>,
}

fn nif_warning(nif: Option<&str>, pais: &str) -> Option<String> {
    let nif = nif?.trim();
    if nif.is_empty() {
        return None;
    }
    if pais.eq_ignore_ascii_case("PT") && !fiscal::validate_nif_pt(nif) {
        Some("NIF PT inválido (check-digit)".into())
    } else {
        None
    }
}

async fn create_customer(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCustomerRequest>,
) -> ApiResult<(StatusCode, Json<CustomerResponse>)> {
    let pais = req.pais.clone().unwrap_or_else(|| "PT".into());
    let warning = nif_warning(req.nif.as_deref(), &pais);
    let c = storage::create_customer(
        state.db.pool(),
        storage::NewCustomer {
            nome: req.nome,
            nif: req.nif,
            pais: req.pais,
            telefone: req.telefone,
            morada: req.morada,
            cod_postal: req.cod_postal,
            localidade: req.localidade,
            email: req.email,
            observacoes: req.observacoes,
            zona_id: req.zona_id,
        },
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(CustomerResponse { customer: c, nif_warning: warning }),
    ))
}

#[derive(Deserialize, Default)]
pub struct UpdateCustomerRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub nome: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub nif: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub pais: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub telefone: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub morada: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub cod_postal: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub localidade: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub email: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub observacoes: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub zona_id: OptionalField<Option<Uuid>>,
}

async fn update_customer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCustomerRequest>,
) -> ApiResult<Json<CustomerResponse>> {
    let pais_set = match &req.pais {
        OptionalField::Set(v) => Some(v.clone()),
        OptionalField::Missing => None,
    };
    let upd = storage::CustomerUpdate {
        nome: req.nome.into_option(),
        nif: req.nif.into_option(),
        pais: req.pais.into_option(),
        telefone: req.telefone.into_option(),
        morada: req.morada.into_option(),
        cod_postal: req.cod_postal.into_option(),
        localidade: req.localidade.into_option(),
        email: req.email.into_option(),
        observacoes: req.observacoes.into_option(),
        zona_id: req.zona_id.into_option(),
    };
    let nif_new = upd.nif.clone();
    let c = storage::update_customer(state.db.pool(), id, upd).await?;
    let pais_effective = pais_set.unwrap_or_else(|| c.pais.clone());
    let nif_check = match nif_new {
        Some(opt) => opt,
        None => c.nif.clone(),
    };
    let warning = nif_warning(nif_check.as_deref(), &pais_effective);
    Ok(Json(CustomerResponse { customer: c, nif_warning: warning }))
}

async fn forget_customer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Customer>> {
    Ok(Json(storage::forget_customer(state.db.pool(), id).await?))
}

#[derive(Deserialize, Default)]
pub struct StartLocalDocumentRequest {
    pub employee_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub observacoes_pedido: Option<String>,
}

async fn start_local_document(
    State(state): State<Arc<AppState>>,
    Path(local_id): Path<Uuid>,
    body: Option<Json<StartLocalDocumentRequest>>,
) -> ApiResult<Json<DocumentResponse>> {
    let body = body.map(|b| b.0).unwrap_or_default();
    let pool = state.db.pool();
    let local = storage::get_local(pool, local_id).await?;
    let business_date = state.config.business_day(Utc::now());
    let document =
        storage::start_document_for_local(pool, local_id, body.employee_id, business_date).await?;

    let mut customer_snapshot: Option<Customer> = None;
    if let Some(cid) = body.customer_id {
        customer_snapshot = Some(storage::get_customer(pool, cid).await?);
    }

    if customer_snapshot.is_some() || body.observacoes_pedido.is_some() {
        let upd = storage::DocumentContextUpdate {
            customer_id: body.customer_id.map(Some),
            observacoes_pedido: body.observacoes_pedido.clone().map(Some),
            delivery_morada: customer_snapshot
                .as_ref()
                .and_then(|c| c.morada.clone())
                .map(Some),
            delivery_telefone: customer_snapshot
                .as_ref()
                .and_then(|c| c.telefone.clone())
                .map(Some),
            ..Default::default()
        };
        storage::update_document_context(pool, document.id, upd).await?;
    }

    // Delivery: cria o pedido_delivery + adiciona linha de taxa de entrega se o cliente
    // tem zona com taxa configurada.
    if matches!(local.tipo, domain::LocalKind::Delivery) {
        let (zona_id, taxa) = if let Some(c) = customer_snapshot.as_ref() {
            if let Some(zid) = c.zona_id {
                let z = storage::get_zona(pool, zid).await?;
                (Some(zid), z.taxa_entrega)
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };
        storage::create_pedido_delivery(
            pool,
            document.id,
            body.customer_id,
            customer_snapshot.as_ref().and_then(|c| c.morada.clone()),
            customer_snapshot.as_ref().and_then(|c| c.telefone.clone()),
            "balcao",
            zona_id,
            taxa,
        )
        .await?;
        if taxa > 0 {
            // Procurar artigo "Taxa de Entrega" (code 9999) e adicionar 1 unidade.
            let articles = storage::list_articles(pool).await?;
            if let Some(art) = articles.into_iter().find(|a| a.code == 9999) {
                storage::add_document_line(pool, document.id, art.id, 1, taxa).await?;
            }
        }
    }

    let document = storage::get_document(pool, document.id).await?;
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentCreated { document_id: document.id });
    Ok(Json(build_doc_response(pool, document).await?))
}

#[derive(Deserialize)]
pub struct OpenConsumoRequest {
    pub employee_id: Uuid,
}

async fn open_consumo_proprio(
    State(state): State<Arc<AppState>>,
    Path(local_id): Path<Uuid>,
    Json(req): Json<OpenConsumoRequest>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let local = storage::get_local(pool, local_id).await?;
    if !matches!(local.tipo, domain::LocalKind::ConsumoProprio) {
        return Err(ApiError::BadRequest(
            "este endpoint é só para locais consumo_proprio".into(),
        ));
    }
    let employee = storage::get_employee(pool, req.employee_id).await?;
    let table = storage::ensure_consumo_table(pool, local_id, &employee).await?;
    let business_date = state.config.business_day(Utc::now());
    let document =
        storage::open_table(pool, table.id, Some(employee.id), business_date).await?;
    Ok(Json(build_doc_response(pool, document).await?))
}

#[derive(Deserialize, Default)]
pub struct DocumentContextRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub customer_id: OptionalField<Option<Uuid>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub observacoes_pedido: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub observacoes_factura: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub observacoes_cliente: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub observacoes_morada: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub delivery_morada: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub delivery_telefone: OptionalField<Option<String>>,
}

async fn set_document_context(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<DocumentContextRequest>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let upd = storage::DocumentContextUpdate {
        customer_id: req.customer_id.into_option(),
        observacoes_pedido: req.observacoes_pedido.into_option(),
        observacoes_factura: req.observacoes_factura.into_option(),
        observacoes_cliente: req.observacoes_cliente.into_option(),
        observacoes_morada: req.observacoes_morada.into_option(),
        delivery_morada: req.delivery_morada.into_option(),
        delivery_telefone: req.delivery_telefone.into_option(),
    };
    let doc = storage::update_document_context(pool, id, upd).await?;
    Ok(Json(build_doc_response(pool, doc).await?))
}

async fn get_active_deliveries(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<PedidoDelivery>>> {
    Ok(Json(
        storage::list_active_pedidos_delivery(state.db.pool()).await?,
    ))
}

#[derive(Deserialize)]
pub struct UpdateDeliveryStateRequest {
    pub estado: String,
    pub entregador_id: Option<Uuid>,
}

async fn update_delivery_state(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDeliveryStateRequest>,
) -> ApiResult<Json<PedidoDelivery>> {
    let estado = DeliveryEstado::parse(&req.estado)
        .ok_or_else(|| ApiError::BadRequest(format!("estado inválido: {}", req.estado)))?;
    Ok(Json(
        storage::update_delivery_estado(state.db.pool(), id, estado, req.entregador_id).await?,
    ))
}

async fn get_tipos_preco(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<TipoPreco>>> {
    Ok(Json(storage::list_tipos_preco(state.db.pool()).await?))
}

async fn get_zonas(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Zona>>> {
    Ok(Json(storage::list_zonas(state.db.pool()).await?))
}

#[derive(Deserialize)]
pub struct CreateZonaRequest {
    pub designacao: String,
    pub codigo: Option<i32>,
    #[serde(default)]
    pub taxa_entrega: i64,
}

async fn create_zona(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateZonaRequest>,
) -> ApiResult<(StatusCode, Json<Zona>)> {
    let z = storage::create_zona(
        state.db.pool(),
        storage::NewZona {
            designacao: req.designacao,
            codigo: req.codigo,
            taxa_entrega: req.taxa_entrega,
        },
    )
    .await?;
    Ok((StatusCode::CREATED, Json(z)))
}

#[derive(Deserialize, Default)]
pub struct UpdateZonaRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub designacao: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub codigo: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub taxa_entrega: OptionalField<i64>,
}

async fn update_zona(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateZonaRequest>,
) -> ApiResult<Json<Zona>> {
    let upd = storage::ZonaUpdate {
        designacao: req.designacao.into_option(),
        codigo: req.codigo.into_option(),
        taxa_entrega: req.taxa_entrega.into_option(),
    };
    Ok(Json(storage::update_zona(state.db.pool(), id, upd).await?))
}

async fn delete_zona(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_zona(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_entregadores(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<Entregador>>> {
    Ok(Json(storage::list_entregadores(state.db.pool()).await?))
}

#[derive(Deserialize)]
pub struct CreateEntregadorRequest {
    pub nome: String,
    pub telefone: Option<String>,
    #[serde(default = "default_externo")]
    pub externo: bool,
}

fn default_externo() -> bool { true }

async fn create_entregador(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateEntregadorRequest>,
) -> ApiResult<(StatusCode, Json<Entregador>)> {
    let e = storage::create_entregador(
        state.db.pool(),
        storage::NewEntregador {
            nome: req.nome,
            telefone: req.telefone,
            externo: req.externo,
        },
    )
    .await?;
    Ok((StatusCode::CREATED, Json(e)))
}

#[derive(Deserialize, Default)]
pub struct UpdateEntregadorRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub nome: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub telefone: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub externo: OptionalField<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub ativo: OptionalField<bool>,
}

async fn update_entregador(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEntregadorRequest>,
) -> ApiResult<Json<Entregador>> {
    let upd = storage::EntregadorUpdate {
        nome: req.nome.into_option(),
        telefone: req.telefone.into_option(),
        externo: req.externo.into_option(),
        ativo: req.ativo.into_option(),
    };
    Ok(Json(
        storage::update_entregador(state.db.pool(), id, upd).await?,
    ))
}

async fn delete_entregador(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_entregador(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_zonas_impressao(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<ZonaImpressao>>> {
    Ok(Json(storage::list_zonas_impressao(state.db.pool()).await?))
}

#[derive(Deserialize)]
pub struct CreateZonaImpressaoRequest {
    pub codigo: i32,
    pub designacao: String,
    #[serde(default)]
    pub secundarios: bool,
}

async fn create_zona_impressao(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateZonaImpressaoRequest>,
) -> ApiResult<(StatusCode, Json<ZonaImpressao>)> {
    let z = storage::create_zona_impressao(
        state.db.pool(),
        storage::NewZonaImpressao {
            codigo: req.codigo,
            designacao: req.designacao,
            secundarios: req.secundarios,
        },
    )
    .await?;
    Ok((StatusCode::CREATED, Json(z)))
}

#[derive(Deserialize, Default)]
pub struct UpdateZonaImpressaoRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub codigo: OptionalField<i32>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub designacao: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub secundarios: OptionalField<bool>,
}

async fn update_zona_impressao(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateZonaImpressaoRequest>,
) -> ApiResult<Json<ZonaImpressao>> {
    let upd = storage::ZonaImpressaoUpdate {
        codigo: req.codigo.into_option(),
        designacao: req.designacao.into_option(),
        secundarios: req.secundarios.into_option(),
    };
    Ok(Json(
        storage::update_zona_impressao(state.db.pool(), id, upd).await?,
    ))
}

async fn delete_zona_impressao(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_zona_impressao(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_dispositivos(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<Dispositivo>>> {
    Ok(Json(storage::list_dispositivos(state.db.pool()).await?))
}

#[derive(Deserialize)]
pub struct CreateDispositivoRequest {
    pub nome: String,
    #[serde(default = "default_tipo_disp")]
    pub tipo: String,
    pub modelo: Option<String>,
    pub descricao: Option<String>,
    pub output_path: Option<String>,
    #[serde(default = "default_conexao_tipo")]
    pub conexao_tipo: String,
    #[serde(default)]
    pub conexao_config: serde_json::Value,
}

fn default_tipo_disp() -> String {
    "impressora_generica".into()
}

fn default_conexao_tipo() -> String {
    "file".into()
}

async fn create_dispositivo(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateDispositivoRequest>,
) -> ApiResult<(StatusCode, Json<Dispositivo>)> {
    let d = storage::create_dispositivo(
        state.db.pool(),
        storage::NewDispositivo {
            nome: req.nome,
            tipo: req.tipo,
            modelo: req.modelo,
            descricao: req.descricao,
            output_path: req.output_path,
            conexao_tipo: req.conexao_tipo,
            conexao_config: req.conexao_config,
        },
    )
    .await?;
    Ok((StatusCode::CREATED, Json(d)))
}

#[derive(Deserialize, Default)]
pub struct UpdateDispositivoRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub nome: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub tipo: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub modelo: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub descricao: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub output_path: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub ativo: OptionalField<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub conexao_tipo: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub conexao_config: OptionalField<serde_json::Value>,
}

async fn update_dispositivo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDispositivoRequest>,
) -> ApiResult<Json<Dispositivo>> {
    let upd = storage::DispositivoUpdate {
        nome: req.nome.into_option(),
        tipo: req.tipo.into_option(),
        modelo: req.modelo.into_option(),
        descricao: req.descricao.into_option(),
        output_path: req.output_path.into_option(),
        ativo: req.ativo.into_option(),
        conexao_tipo: req.conexao_tipo.into_option(),
        conexao_config: req.conexao_config.into_option(),
    };
    Ok(Json(
        storage::update_dispositivo(state.db.pool(), id, upd).await?,
    ))
}

async fn delete_dispositivo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_dispositivo(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct DeviceStatusDto {
    pub health: String,
    pub queued: usize,
    pub last_error: Option<String>,
    pub jobs_done: u64,
}

async fn get_dispositivo_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<DeviceStatusDto>> {
    use devices::spooler::DeviceHealth;
    let s = state.spooler.status(&id.to_string()).await.unwrap_or_default();
    let health = match s.health {
        DeviceHealth::Ok => "ok",
        DeviceHealth::Failed => "failed",
        DeviceHealth::Unknown => "unknown",
    };
    Ok(Json(DeviceStatusDto {
        health: health.into(),
        queued: s.queued,
        last_error: s.last_error,
        jobs_done: s.jobs_done,
    }))
}

/// Envia um talão de teste para validar a ligação do dispositivo.
async fn test_dispositivo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let d = storage::get_dispositivo(state.db.pool(), id).await?;
    let ticket = format!(
        "*** TESTE OpenRest ***\n{}\n{}\nLigacao: {}\n",
        d.nome,
        Utc::now().format("%Y-%m-%d %H:%M:%S"),
        d.conexao_tipo
    );
    enqueue_ticket(&state, &d, &ticket).await;
    Ok(StatusCode::ACCEPTED)
}

async fn get_print_mappings(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<ImpressoraZonaLocal>>> {
    Ok(Json(storage::list_print_mappings(state.db.pool()).await?))
}

#[derive(Deserialize)]
pub struct CreateMappingRequest {
    pub zona_impressao_id: Uuid,
    pub local_id: Uuid,
    pub origem_id: Option<Uuid>,
    pub dispositivo_id: Uuid,
    #[serde(default = "default_agrupamento")]
    pub agrupamento: String,
    #[serde(default = "default_copias")]
    pub numero_copias: i32,
}

fn default_agrupamento() -> String { "normal".into() }
fn default_copias() -> i32 { 1 }

async fn create_print_mapping(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateMappingRequest>,
) -> ApiResult<(StatusCode, Json<ImpressoraZonaLocal>)> {
    let m = storage::create_print_mapping(
        state.db.pool(),
        storage::NewMapping {
            zona_impressao_id: req.zona_impressao_id,
            local_id: req.local_id,
            origem_id: req.origem_id,
            dispositivo_id: req.dispositivo_id,
            agrupamento: req.agrupamento,
            numero_copias: req.numero_copias,
        },
    )
    .await?;
    Ok((StatusCode::CREATED, Json(m)))
}

async fn delete_print_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_print_mapping(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// "Pedir": agrupa as linhas pendentes por zona de impressão e imprime no dispositivo
/// configurado para cada (zona, local). Linhas sem zona caem na zona "Documentos Externos"
/// — não são impressas aqui (vão no documento legal).
async fn pedir_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<DocumentResponse>> {
    use std::collections::BTreeMap;

    let pool = state.db.pool();
    let document = storage::get_document(pool, id).await?;
    let local_id = document
        .local_id
        .ok_or_else(|| ApiError::BadRequest("documento sem local associado".into()))?;
    let local = storage::get_local(pool, local_id).await?;
    let table_label = match document.table_id {
        Some(tid) => storage::get_table(pool, tid)
            .await
            .ok()
            .and_then(|t| t.name.or(Some(format!("Mesa {}", t.code))))
            .unwrap_or_else(|| "Mesa".into()),
        None => "Balcão".into(),
    };

    let lines = storage::list_document_details(pool, document.id).await?;
    let pending: Vec<&DocumentDetail> = lines.iter().filter(|l| l.pedida_em.is_none()).collect();
    if pending.is_empty() {
        return Ok(Json(build_doc_response(pool, document).await?));
    }

    let mut articles_by_id = std::collections::HashMap::new();
    for line in &pending {
        if !articles_by_id.contains_key(&line.article_id) {
            let a = storage::get_article(pool, line.article_id).await?;
            articles_by_id.insert(line.article_id, a);
        }
    }

    let mut by_zone: BTreeMap<Uuid, Vec<&DocumentDetail>> = BTreeMap::new();
    let mut without_zone: Vec<&DocumentDetail> = Vec::new();
    for line in &pending {
        let art = &articles_by_id[&line.article_id];
        if let Some(z) = art.zona_impressao_id {
            by_zone.entry(z).or_default().push(*line);
        } else {
            without_zone.push(*line);
        }
    }

    // Pré-carrega todas as zonas envolvidas (para saber quais são secundárias).
    let mut zonas: std::collections::HashMap<Uuid, ZonaImpressao> =
        std::collections::HashMap::new();
    for zid in by_zone.keys() {
        zonas.insert(*zid, storage::get_zona_impressao(pool, *zid).await?);
    }

    // Conjunto de zonas secundárias com linhas neste lote — entre estas há
    // espelho cruzado ("sai junto com") (spec 03/05 §4).
    let secondary_zones: Vec<Uuid> = by_zone
        .keys()
        .filter(|z| zonas.get(z).map(|z| z.secundarios).unwrap_or(false))
        .copied()
        .collect();

    let now = chrono::Utc::now();
    let mut printed_line_ids: Vec<Uuid> = Vec::new();

    for (zona_id, ls) in by_zone.iter() {
        let Some(dispositivo) =
            storage::dispositivo_for_zona_local(pool, *zona_id, local.id).await?
        else {
            tracing::warn!(
                "sem mapping para zona {} no local {}; linhas saltadas",
                zona_id,
                local.id
            );
            continue;
        };
        let zona = &zonas[zona_id];
        let kitchen_lines: Vec<devices::escpos::KitchenLine> = ls
            .iter()
            .map(|l| devices::escpos::KitchenLine {
                qty: l.qty,
                name: &articles_by_id[&l.article_id].name,
            })
            .collect();

        // Se esta zona é secundária e há mais de uma zona secundária no lote,
        // anexa um bloco "sai junto com" por cada uma das outras zonas secundárias.
        let mut cross_storage: Vec<(String, Vec<devices::escpos::KitchenLine>)> = Vec::new();
        if zona.secundarios {
            for other_id in secondary_zones.iter() {
                if other_id == zona_id {
                    continue;
                }
                let other_zona = &zonas[other_id];
                let other_lines: Vec<devices::escpos::KitchenLine> = by_zone[other_id]
                    .iter()
                    .map(|l| devices::escpos::KitchenLine {
                        qty: l.qty,
                        name: &articles_by_id[&l.article_id].name,
                    })
                    .collect();
                cross_storage.push((other_zona.designacao.clone(), other_lines));
            }
        }
        let cross_blocks: Vec<devices::escpos::CrossZoneBlock> = cross_storage
            .iter()
            .map(|(name, lines)| devices::escpos::CrossZoneBlock {
                zona: name,
                lines,
            })
            .collect();

        let ticket = devices::escpos::format_kitchen_ticket(
            &zona.designacao,
            &local.designacao,
            &table_label,
            now,
            &kitchen_lines,
            &cross_blocks,
        );
        enqueue_ticket(&state, &dispositivo, &ticket).await;
        for l in ls {
            printed_line_ids.push(l.id);
        }
    }
    // Linhas sem zona ainda são marcadas como pedidas (não imprimem ticket de cozinha).
    for l in &without_zone {
        printed_line_ids.push(l.id);
    }

    storage::mark_lines_pedidas(pool, document.id, &printed_line_ids).await?;
    let document = storage::get_document(pool, document.id).await?;
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentLineAdded { document_id: document.id });
    Ok(Json(build_doc_response(pool, document).await?))
}

/// Resolve o transporte de um dispositivo a partir da sua configuração.
fn device_connection(d: &Dispositivo) -> Result<devices::transport::Connection, ApiError> {
    devices::transport::Connection::from_config(&d.conexao_tipo, &d.conexao_config)
        .map_err(|e| ApiError::BadRequest(e.to_string()))
}

/// Bytes a enviar a um dispositivo: ligações `file`/`null` recebem texto
/// legível; as restantes recebem ESC/POS codificado (codepage + corte).
fn bytes_for_device(d: &Dispositivo, text: &str) -> Vec<u8> {
    match d.conexao_tipo.as_str() {
        "file" | "null" => text.as_bytes().to_vec(),
        _ => devices::escpos_encode::encode(
            text,
            &devices::escpos_encode::EscposProfile::default(),
        ),
    }
}

/// Coloca um talão na fila do dispositivo (não bloqueia; o spooler trata de
/// retry e estado). Ligações inválidas são registadas, não falham o pedido.
async fn enqueue_ticket(state: &AppState, dispositivo: &Dispositivo, ticket: &str) {
    match device_connection(dispositivo) {
        Ok(conn) => {
            let bytes = bytes_for_device(dispositivo, ticket);
            state
                .spooler
                .enqueue(&dispositivo.id.to_string(), conn, bytes, 1)
                .await;
        }
        Err(e) => {
            tracing::warn!("dispositivo {} sem ligação válida: {}", dispositivo.id, e);
        }
    }
}

async fn print_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let pool = state.db.pool();
    let document = storage::get_document(pool, id).await?;
    if !document.is_closed {
        return Err(ApiError::BadRequest("document not closed".into()));
    }
    let lines = storage::list_document_details(pool, document.id).await?;
    let payments = storage::list_document_payments(pool, document.id).await?;

    let mut articles = Vec::with_capacity(lines.len());
    for l in &lines {
        articles.push(storage::get_article(pool, l.article_id).await?);
    }
    let breakdown = compute_vat_breakdown(&lines, &articles);

    let table_label = match document.table_id {
        Some(tid) => storage::get_table(pool, tid)
            .await
            .ok()
            .and_then(|t| t.name.or(Some(format!("Mesa {}", t.code))))
            .unwrap_or_else(|| "Mesa".to_string()),
        None => "Balcão".to_string(),
    };

    let payment_methods = storage::list_payment_methods(pool).await?;
    let payments_with_label: Vec<(String, i64)> = payments
        .iter()
        .map(|p| {
            let method = payment_methods
                .iter()
                .find(|m| m.id == p.payment_method_id)
                .map(|m| m.name.clone())
                .unwrap_or_else(|| "?".into());
            // Anexa a descrição livre (e.g., "Visa **1234") ao rótulo quando preenchida.
            let label = match p.descricao.as_deref() {
                Some(d) if !d.is_empty() => format!("{} {}", method, d),
                _ => method,
            };
            (label, p.amount)
        })
        .collect();

    // Cliente / empregado (snapshots para as flags do cabeçalho).
    let customer = match document.customer_id {
        Some(cid) => storage::get_customer(pool, cid).await.ok(),
        None => None,
    };
    let employee = match document.employee_id {
        Some(eid) => storage::get_employee(pool, eid).await.ok(),
        None => None,
    };

    let qr_block = document
        .qr_payload
        .as_deref()
        .and_then(|p| fiscal::render_qr_ascii(p).ok())
        .unwrap_or_default();

    // Selecciona o template configurável para o tipo de documento; cai no de
    // factura simplificada se o tipo não tiver template próprio.
    let tipo = template_tipo_for(document.document_type.as_deref());
    let tpl = match storage::get_document_template(pool, tipo).await {
        Ok(t) => t,
        Err(storage::StorageError::NotFound) => {
            storage::get_document_template(pool, "fatura_simplificada").await?
        }
        Err(e) => return Err(e.into()),
    };

    let ctx = build_document_context(
        &state.config,
        &document,
        &lines,
        &articles,
        &breakdown,
        &payments_with_label,
        &table_label,
        customer.as_ref(),
        employee.as_ref(),
        qr_block,
    );

    let width = if tpl.largura > 0 { tpl.largura as usize } else { devices::template::DEFAULT_WIDTH };
    let tpl_engine = devices::template::DocumentTemplate {
        cabecalho: tpl.cabecalho,
        linha_detalhe: tpl.linha_detalhe,
        rodape: tpl.rodape,
        nao_imprime_detalhes: tpl.nao_imprime_detalhes,
    };
    let receipt = devices::template::render_document(&tpl_engine, &ctx, width);

    let printer = devices::GenericPrinter::new(state.config.printer_output_path.clone());
    printer
        .print_receipt(&receipt)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, receipt))
}

/// Mapeia o código de tipo de documento (série) ao `tipo_documento` do template.
/// Documentos de venda em PT: FS (fatura simplificada), FR (fatura-recibo),
/// FT (fatura a crédito).
fn template_tipo_for(document_type: Option<&str>) -> &'static str {
    match document_type {
        Some("FT") => "fatura",
        Some("FR") => "fatura_recibo",
        _ => "fatura_simplificada", // FS e qualquer outro
    }
}

/// Constrói o contexto de renderização a partir do documento fechado.
#[allow(clippy::too_many_arguments)]
fn build_document_context(
    config: &server::AppConfig,
    document: &Document,
    lines: &[DocumentDetail],
    articles: &[Article],
    breakdown: &[VatBucket],
    payments_with_label: &[(String, i64)],
    table_label: &str,
    customer: Option<&Customer>,
    employee: Option<&Employee>,
    qr_block: String,
) -> devices::template::DocumentContext {
    use devices::template as tpl;
    let c = &config.company;
    let total_sem_iva: i64 = breakdown.iter().map(|b| b.base).sum();
    let iva_total: i64 = breakdown.iter().map(|b| b.vat).sum();
    let pago: i64 = payments_with_label.iter().map(|(_, a)| *a).sum();

    tpl::DocumentContext {
        company: tpl::Company {
            legal_name: c.legal_name.clone(),
            trade_name: c.trade_name.clone(),
            nif: c.nif.clone(),
            address: c.address.clone(),
            city: c.city.clone(),
            postal_code: c.postal_code.clone(),
            country: Some(c.country.clone()),
            phone: None,
            fax: None,
            registry_office: c.registry_office.clone(),
            registry_number: c.registry_number.clone(),
            share_capital_cents: c.share_capital_cents,
        },
        client: customer.map(|cu| tpl::Party {
            name: Some(cu.nome.clone()),
            number: cu.codigo.map(|n| n.to_string()),
            nif: cu.nif.clone(),
            address: cu.morada.clone(),
            city: cu.localidade.clone(),
            postal_code: cu.cod_postal.clone(),
            zone: None,
            association_name: None,
            association_nif: None,
        }),
        employee: tpl::Staff {
            number: employee.map(|e| e.code.to_string()),
            name: employee.map(|e| e.name.clone()),
        },
        table_number: document.table_id.map(|_| table_label.to_string()),
        table_name: Some(table_label.to_string()),
        local_name: None,
        issued_at: Some(document.issued_at.unwrap_or(document.created_at)),
        opened_at: Some(document.created_at),
        now: Some(Utc::now()),
        document_number: document.document_number.map(|n| n.to_string()),
        series: document.document_type.clone(),
        document_type_label: document.document_type.clone(),
        atcud: document.atcud.clone(),
        hash_short: document.hash_short.clone(),
        software_version: Some(env!("CARGO_PKG_VERSION").to_string()),
        software_certificate: Some(c.software_certificate.clone()),
        num_people: None,
        subtotal: document.total,
        total: document.total,
        total_sem_iva,
        iva_total,
        secondary_rate: None,
        payments: payments_with_label
            .iter()
            .map(|(method, amount)| tpl::PaymentLine {
                method: method.clone(),
                amount: *amount,
            })
            .collect(),
        troco: document.troco_cents,
        gorjeta: 0,
        pago,
        a1: None,
        a2: None,
        a3: None,
        lines: lines
            .iter()
            .zip(articles.iter())
            .map(|(l, a)| tpl::LineContext {
                qty_milli: l.qty_milli,
                article_code: Some(a.code.to_string()),
                name: l.descricao.clone().unwrap_or_else(|| a.name.clone()),
                short_name: None,
                unit_price: l.unit_price,
                price_sem_iva: 0,
                perc_desc_bp: 0,
                val_desc: 0,
                iva_cod: None,
                iva_perc_bp: a.vat_rate,
                total: l.total,
                zona_imp: None,
                emp_pedido: None,
                hora: None,
            })
            .collect(),
        vat_rows: breakdown
            .iter()
            .map(|b| tpl::VatRow {
                label: format!("{:.1}%", b.rate_bp as f64 / 100.0),
                base: b.base,
                vat: b.vat,
            })
            .collect(),
        qr_block,
        qr_payload: document.qr_payload.clone().unwrap_or_default(),
    }
}

async fn get_document_templates(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<DocumentTemplate>>> {
    Ok(Json(storage::list_document_templates(state.db.pool()).await?))
}

async fn get_document_template_handler(
    State(state): State<Arc<AppState>>,
    Path(tipo): Path<String>,
) -> ApiResult<Json<DocumentTemplate>> {
    Ok(Json(
        storage::get_document_template(state.db.pool(), &tipo).await?,
    ))
}

#[derive(Deserialize)]
pub struct UpdateDocumentTemplateRequest {
    pub designacao: String,
    pub cabecalho: String,
    pub linha_detalhe: String,
    pub rodape: String,
    #[serde(default)]
    pub nao_imprime_detalhes: bool,
    #[serde(default = "default_template_width")]
    pub largura: i32,
}

fn default_template_width() -> i32 {
    devices::template::DEFAULT_WIDTH as i32
}

async fn update_document_template_handler(
    State(state): State<Arc<AppState>>,
    Path(tipo): Path<String>,
    Json(req): Json<UpdateDocumentTemplateRequest>,
) -> ApiResult<Json<DocumentTemplate>> {
    let pool = state.db.pool();
    // Reutiliza o id existente quando o template já existe (upsert por tipo).
    let id = storage::get_document_template(pool, &tipo)
        .await
        .map(|t| t.id)
        .unwrap_or_else(|_| Uuid::new_v4());
    let tpl = DocumentTemplate {
        id,
        tipo_documento: tipo,
        designacao: req.designacao,
        cabecalho: req.cabecalho,
        linha_detalhe: req.linha_detalhe,
        rodape: req.rodape,
        nao_imprime_detalhes: req.nao_imprime_detalhes,
        largura: req.largura,
        anulado_em: None,
    };
    Ok(Json(storage::upsert_document_template(pool, &tpl).await?))
}


#[cfg(test)]
mod template_tests {
    use chrono::{TimeZone, Utc};
    use devices::template as tpl;
    use uuid::Uuid;

    async fn migrated_db() -> storage::Database {
        let path = std::env::temp_dir().join(format!("openrest_tpl_{}.db", Uuid::new_v4()));
        let url = format!("sqlite://{}", path.display());
        let db = storage::Database::new(&url).await.unwrap();
        db.migrate().await.unwrap();
        db
    }

    fn sample_ctx() -> tpl::DocumentContext {
        tpl::DocumentContext {
            company: tpl::Company {
                legal_name: "Tasca do Zé, Lda".into(),
                trade_name: Some("Tasca do Zé".into()),
                nif: "501234567".into(),
                address: "Rua Direita, 10".into(),
                city: Some("Porto".into()),
                postal_code: Some("4000-001".into()),
                ..Default::default()
            },
            total: 1130,
            subtotal: 1130,
            total_sem_iva: 1000,
            iva_total: 130,
            document_number: Some("12".into()),
            series: Some("FS".into()),
            atcud: Some("AT-XYZ-12".into()),
            hash_short: Some("AB12".into()),
            issued_at: Some(Utc.with_ymd_and_hms(2026, 5, 29, 14, 30, 0).unwrap()),
            pago: 1130,
            lines: vec![tpl::LineContext {
                qty_milli: 1000,
                name: "Café".into(),
                unit_price: 80,
                total: 80,
                iva_perc_bp: 1300,
                ..Default::default()
            }],
            vat_rows: vec![tpl::VatRow { label: "13%".into(), base: 1000, vat: 130 }],
            payments: vec![tpl::PaymentLine { method: "Numerário".into(), amount: 1130 }],
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn seeded_fatura_simplificada_renders() {
        let db = migrated_db().await;
        let t = storage::get_document_template(db.pool(), "fatura_simplificada")
            .await
            .unwrap();
        let engine = tpl::DocumentTemplate {
            cabecalho: t.cabecalho,
            linha_detalhe: t.linha_detalhe,
            rodape: t.rodape,
            nao_imprime_detalhes: t.nao_imprime_detalhes,
        };
        let out = tpl::render_document(&engine, &sample_ctx(), t.largura as usize);
        assert!(out.contains("Tasca do Zé"), "header trade name: {out}");
        assert!(out.contains("Factura Simplificada"), "title: {out}");
        assert!(out.contains("FS/12"), "doc identifier: {out}");
        assert!(out.contains("Café"), "detail line: {out}");
        assert!(out.contains("TOTAL"), "total: {out}");
        assert!(out.contains("ATCUD: AT-XYZ-12"), "atcud: {out}");
        assert!(out.contains("13%"), "vat table: {out}");
    }

    #[tokio::test]
    async fn all_default_templates_present() {
        let db = migrated_db().await;
        let all = storage::list_document_templates(db.pool()).await.unwrap();
        for tipo in ["fatura_simplificada", "fatura", "fatura_recibo", "consulta_mesa", "pedido"] {
            assert!(all.iter().any(|t| t.tipo_documento == tipo), "missing {tipo}");
        }
    }

    #[tokio::test]
    async fn upsert_roundtrip_keeps_id() {
        let db = migrated_db().await;
        let mut t = storage::get_document_template(db.pool(), "fatura")
            .await
            .unwrap();
        let original_id = t.id;
        t.cabecalho = r"\s7\no -- EDITADO".into();
        let saved = storage::upsert_document_template(db.pool(), &t).await.unwrap();
        assert_eq!(saved.cabecalho, r"\s7\no -- EDITADO");
        assert_eq!(saved.id, original_id);
        let again = storage::get_document_template(db.pool(), "fatura")
            .await
            .unwrap();
        assert_eq!(again.cabecalho, r"\s7\no -- EDITADO");
        assert_eq!(again.id, original_id);
    }

    fn ctx_with_client() -> tpl::DocumentContext {
        let mut c = sample_ctx();
        c.client = Some(tpl::Party {
            name: Some("João Silva".into()),
            number: Some("42".into()),
            nif: Some("245678901".into()),
            address: Some("Av. da Liberdade, 22".into()),
            city: Some("Lisboa".into()),
            postal_code: Some("1250-096".into()),
            ..Default::default()
        });
        c
    }

    fn engine_of(t: domain::DocumentTemplate) -> tpl::DocumentTemplate {
        tpl::DocumentTemplate {
            cabecalho: t.cabecalho,
            linha_detalhe: t.linha_detalhe,
            rodape: t.rodape,
            nao_imprime_detalhes: t.nao_imprime_detalhes,
        }
    }

    #[tokio::test]
    async fn fatura_renders_client_and_unit_price() {
        let db = migrated_db().await;
        let t = storage::get_document_template(db.pool(), "fatura")
            .await
            .unwrap();
        let width = t.largura as usize;
        let out = tpl::render_document(&engine_of(t), &ctx_with_client(), width);
        assert!(out.contains("FACTURA  (Original)"), "title/original: {out}");
        // Bloco de cliente nominativo.
        assert!(out.contains("João Silva"), "client name: {out}");
        assert!(out.contains("NIF: 245678901"), "client nif: {out}");
        assert!(out.contains("Av. da Liberdade, 22"), "client address: {out}");
        assert!(out.contains("1250-096 Lisboa"), "client postal/city: {out}");
        // Cabeçalho de colunas com preço unitário e linha de detalhe alinhada.
        assert!(out.contains("P.Unit"), "unit price header: {out}");
        assert!(out.contains("Café"), "detail: {out}");
        // Larguras consistentes: separadores e linhas com a largura do template.
        for line in out.lines() {
            assert!(line.chars().count() <= width, "linha excede largura: {line:?}");
        }
        assert!(out.contains("ATCUD: AT-XYZ-12"), "atcud: {out}");
        // FT é a crédito: não exibe bloco de pagamento recebido.
        assert!(out.contains("a crédito"), "natureza a crédito: {out}");
        assert!(!out.contains("Total pago"), "FT não deve mostrar pagamento: {out}");
    }

    #[tokio::test]
    async fn fatura_recibo_renders_payment_block() {
        let db = migrated_db().await;
        let t = storage::get_document_template(db.pool(), "fatura_recibo")
            .await
            .unwrap();
        let width = t.largura as usize;
        let mut ctx = ctx_with_client();
        ctx.troco = 70; // 0.70
        let out = tpl::render_document(&engine_of(t), &ctx, width);
        assert!(out.contains("FACTURA-RECIBO"), "title: {out}");
        // Nominativo: bloco de cliente.
        assert!(out.contains("João Silva"), "client: {out}");
        // Recibo: bloco de pagamento recebido.
        assert!(out.contains("Forma de pagamento: Numerário"), "forma: {out}");
        assert!(out.contains("Total pago: 11.30"), "pago: {out}");
        assert!(out.contains("Troco: 0.70"), "troco: {out}");
        assert!(out.contains("Recebi(emos)"), "quitação: {out}");
        assert!(out.contains("13%"), "vat table: {out}");
    }
}
