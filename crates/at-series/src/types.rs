//! Tipos request/response correspondentes ao schema XSD da WSDL `series.wsdl`.
//! Estes structs são deliberadamente próximos ao XML — `chrono::NaiveDate`
//! para datas e `String` para campos de comprimento fixo (validação fica
//! delegada à AT que valida e devolve `codResultOper`/`msgResultOper`).

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Tipo da série (`tipoSerie`). Valores documentados pela AT:
/// * `N` — Normal
/// * `T` — Substituição (autofacturação)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TipoSerie {
    Normal,
    Substituicao,
}

impl TipoSerie {
    pub fn as_str(&self) -> &'static str {
        match self {
            TipoSerie::Normal => "N",
            TipoSerie::Substituicao => "T",
        }
    }
}

/// Classe do documento (`classeDoc`, 2 chars). Valores típicos da AT:
/// * `SI` — Documentos relativos à facturação
/// * `MG` — Documentos de movimentação de mercadorias
/// * `RG` — Recibos emitidos
/// * `WD` — Documentos de conferência (Working documents)
pub type ClasseDoc = String;

/// Tipo do documento (`tipoDoc`, 2 chars). Para SI: `FT`, `FS`, `FR`, `NC`, …
pub type TipoDoc = String;

/// Meio de processamento (`meioProcessamento`, 2 chars):
/// * `PF` — Programa de facturação
/// * `OO` — Outras aplicações
/// * `MD` — Manual (pré-impresso)
pub type MeioProcessamento = String;

/// Motivo de anulação (`motivo`, 2 chars). Documentado pela AT.
pub type MotivoAnulacao = String;

/// Estado da série na resposta:
/// * `A` — Activa
/// * `F` — Finalizada
/// * `N` — Anulada
pub type EstadoSerie = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistarSerieRequest {
    pub serie: String,
    pub tipo_serie: TipoSerie,
    pub classe_doc: ClasseDoc,
    pub tipo_doc: TipoDoc,
    /// Início da numeração sequencial (>= 1).
    pub num_inicial_seq: u64,
    pub data_inicio_prev_utiliz: NaiveDate,
    /// Número do certificado AT do software de facturação. `0` se não
    /// aplicável (caso comum em desenvolvimento).
    pub num_cert_sw_fatur: u32,
    pub meio_processamento: MeioProcessamento,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsultarSeriesRequest {
    pub serie: Option<String>,
    pub tipo_serie: Option<TipoSerie>,
    pub classe_doc: Option<ClasseDoc>,
    pub tipo_doc: Option<TipoDoc>,
    pub cod_validacao_serie: Option<String>,
    pub data_registo_de: Option<NaiveDate>,
    pub data_registo_ate: Option<NaiveDate>,
    pub estado: Option<EstadoSerie>,
    pub meio_processamento: Option<MeioProcessamento>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizarSerieRequest {
    pub serie: String,
    pub classe_doc: ClasseDoc,
    pub tipo_doc: TipoDoc,
    pub cod_validacao_serie: String,
    pub seq_ultimo_doc_emitido: u64,
    pub justificacao: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnularSerieRequest {
    pub serie: String,
    pub classe_doc: ClasseDoc,
    pub tipo_doc: TipoDoc,
    pub cod_validacao_serie: String,
    pub motivo: MotivoAnulacao,
    /// Declaração obrigatória de que o sujeito passivo tem conhecimento de
    /// que não deve anular se já emitiu documentos com a série.
    pub declaracao_nao_emissao: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesInfo {
    pub serie: String,
    pub tipo_serie: String,
    pub classe_doc: String,
    pub tipo_doc: String,
    pub num_inicial_seq: u64,
    pub num_final_seq: Option<u64>,
    pub data_inicio_prev_utiliz: NaiveDate,
    pub seq_ultimo_doc_emitido: Option<u64>,
    pub meio_processamento: String,
    pub num_cert_sw_fatur: u32,
    pub cod_validacao_serie: String,
    pub data_registo: NaiveDate,
    pub estado: EstadoSerie,
    pub motivo_estado: Option<String>,
    pub justificacao: Option<String>,
    pub data_estado: String,
    pub nif_comunicou: String,
}
