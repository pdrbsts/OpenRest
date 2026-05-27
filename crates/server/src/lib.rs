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

#[derive(Clone)]
pub struct AppConfig {
    pub printer_output_path: std::path::PathBuf,
    pub terminal_label: String,
    pub company: CompanyConfig,
    pub signing_key: Arc<RsaPrivateKey>,
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
