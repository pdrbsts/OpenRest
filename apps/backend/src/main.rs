use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use serde::Deserialize;
use server::{AppConfig, AppState, CompanyConfig};
use storage::Database;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
#[serde(default)]
struct BackendConfig {
    bind: String,
    database_path: String,
    printer_output_path: String,
    terminal_label: String,
    signing_key_path: String,
    company: CompanyConfig,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:3000".into(),
            database_path: "./openrest.db".into(),
            printer_output_path: "./receipts.txt".into(),
            terminal_label: "Terminal 1".into(),
            signing_key_path: "./openrest_signing.pem".into(),
            company: CompanyConfig::default(),
        }
    }
}

fn load_config() -> BackendConfig {
    let path = std::env::var("OPENREST_CONFIG").unwrap_or_else(|_| "openrest.toml".into());
    match std::fs::read_to_string(&path) {
        Ok(s) => match toml::from_str(&s) {
            Ok(c) => {
                tracing::info!("loaded config from {path}");
                c
            }
            Err(e) => {
                tracing::warn!("failed to parse {path}: {e}; using defaults");
                BackendConfig::default()
            }
        },
        Err(_) => {
            tracing::info!("no config file at {path}; using defaults");
            BackendConfig::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn")))
        .init();

    let cfg = load_config();

    let db_url = format!("sqlite://{}", cfg.database_path);
    let db = Database::new(&db_url)
        .await
        .with_context(|| format!("connect database at {}", cfg.database_path))?;
    db.migrate().await.context("run migrations")?;

    match storage::seed::seed_if_empty(db.pool()).await {
        Ok(true) => tracing::info!("database seeded with Phase 1 defaults"),
        Ok(false) => tracing::info!("database already populated; skipping seed"),
        Err(e) => tracing::warn!("seeding failed: {e}"),
    }

    let signing_key = tokio::task::spawn_blocking({
        let path = PathBuf::from(&cfg.signing_key_path);
        move || fiscal::load_or_generate_key(&path)
    })
    .await
    .context("signing key task join")?
    .context("load/generate signing key")?;

    let app_config = AppConfig {
        printer_output_path: PathBuf::from(&cfg.printer_output_path),
        terminal_label: cfg.terminal_label,
        company: cfg.company,
        signing_key: Arc::new(signing_key),
    };

    let state = Arc::new(AppState::new(db, app_config));
    let app = api_rest::create_router(state);

    let listener = TcpListener::bind(&cfg.bind).await
        .with_context(|| format!("bind {}", cfg.bind))?;
    tracing::info!("Backend server listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
