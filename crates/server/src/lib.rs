use std::sync::Arc;

use domain::Article;
use rsa::RsaPrivateKey;
use serde::Deserialize;
use storage::Database;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum SystemEvent {
    ArticleCreated(Article),
    ArticleUpdated(Article),
    DocumentCreated { document_id: uuid::Uuid },
    DocumentLineAdded { document_id: uuid::Uuid },
    DocumentClosed { document_id: uuid::Uuid },
}

pub struct EventBus {
    sender: broadcast::Sender<SystemEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.sender.subscribe()
    }

    pub fn publish(
        &self,
        event: SystemEvent,
    ) -> Result<usize, broadcast::error::SendError<SystemEvent>> {
        self.sender.send(event)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CompanyConfig {
    pub nif: String,
    pub legal_name: String,
    #[serde(default)]
    pub trade_name: Option<String>,
    pub address: String,
    #[serde(default)]
    pub postal_code: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default = "default_country")]
    pub country: String,
    #[serde(default)]
    pub share_capital_cents: Option<i64>,
    #[serde(default)]
    pub registry_office: Option<String>,
    #[serde(default)]
    pub registry_number: Option<String>,
    pub software_certificate: String,
    #[serde(default = "default_vat")]
    pub default_vat_rate: i32,
}

fn default_country() -> String {
    "PT".to_string()
}

fn default_vat() -> i32 {
    1300
}

impl Default for CompanyConfig {
    fn default() -> Self {
        Self {
            nif: "999999990".into(),
            legal_name: "OpenRest Demo".into(),
            trade_name: None,
            address: "".into(),
            postal_code: None,
            city: None,
            country: default_country(),
            share_capital_cents: None,
            registry_office: None,
            registry_number: None,
            software_certificate: "0000/AT".into(),
            default_vat_rate: default_vat(),
        }
    }
}

/// Credenciais e endpoint para o web-service `SeriesWS` da AT. Em testes
/// usa-se o endpoint `:722` com NIF público `599999993/0037`; em produção
/// substitui-se tudo (endpoint, credenciais, chave pública).
#[derive(Clone, Debug, Deserialize)]
pub struct AtSeriesConfig {
    pub endpoint: String,
    pub username: String,
    pub password: String,
    pub public_key_path: std::path::PathBuf,
    /// PFX/PKCS#12 do certificado de cliente exigido pelo endpoint TLS da
    /// AT (deve ser emitido por "AT Issuing CA1" ou "DGITA Issuing CA1").
    /// Em testes: `keys/at_test_client.pfx` com a password publicada pela AT.
    pub client_pfx_path: std::path::PathBuf,
    /// Password do PFX. Para o cert público de testes da AT: `TESTEwebservice`.
    #[serde(default)]
    pub client_pfx_password: String,
    #[serde(default)]
    pub num_cert_sw_fatur: u32,
}

#[derive(Clone)]
pub struct AppConfig {
    pub printer_output_path: std::path::PathBuf,
    pub terminal_label: String,
    pub company: CompanyConfig,
    pub signing_key: Arc<RsaPrivateKey>,
    /// Configurável pelo cliente: regista cancelamentos numa tabela de auditoria.
    /// Anulações são sempre registadas (spec).
    pub registar_cancelamentos: bool,
    /// Spec §57 "Data Lógica de Caixa": minutos desde a meia-noite a partir
    /// dos quais já é um novo Dia de facturação. Default 0 (dia civil).
    pub business_day_cutoff_minutes: u32,
    /// Offset do fuso horário da loja em minutos relativos a UTC. Default 0.
    /// Usado para converter o relógio UTC armazenado na hora local da loja
    /// antes de aplicar o corte.
    pub business_day_tz_offset_minutes: i32,
    /// Configuração do cliente AT SeriesWS. `None` desactiva os endpoints
    /// REST relacionados (útil em ambientes onde o ws não está acessível).
    pub at_series: Option<AtSeriesConfig>,
}

impl AppConfig {
    /// Computa o Dia operacional para o instante UTC dado, usando o cutoff e
    /// o offset configurados.
    pub fn business_day(&self, now_utc: chrono::DateTime<chrono::Utc>) -> chrono::NaiveDate {
        let local = domain::utc_to_local(now_utc, self.business_day_tz_offset_minutes);
        domain::compute_business_day(local, self.business_day_cutoff_minutes)
    }
}

pub struct AppState {
    pub event_bus: Arc<EventBus>,
    pub db: Database,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(db: Database, config: AppConfig) -> Self {
        Self {
            event_bus: Arc::new(EventBus::new(1024)),
            db,
            config,
        }
    }
}
