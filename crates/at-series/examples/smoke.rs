//! Smoke test contra o endpoint de testes da AT. Não corre em CI — invoca-se
//! manualmente:
//!
//! ```pwsh
//! cargo run -p at-series --example smoke
//! ```
//!
//! Usa as credenciais públicas de teste (NIF 599999993/0037, password
//! `testes1234`) e a chave pública AT de teste em `keys/at_test_public.pem`.
//!
//! Sequência:
//! 1. `consultarSeries` (read-only) — confirma autenticação + parsing.
//! 2. `registarSerie` para uma série única (carimbo temporal no prefixo)
//!    para evitar colisão com séries existentes do mesmo NIF de teste.

use at_series::{
    ClientConfig, ConsultarSeriesRequest, RegistarSerieRequest, SeriesClient, TipoSerie,
};
use chrono::{Datelike, Utc};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Localiza a chave a partir da raiz do workspace.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap().to_path_buf();
    let pem_path = root.join("keys/at_test_public.pem");
    let pfx_path = root.join("keys/at_test_client.pfx");
    let public_key_pem = std::fs::read_to_string(&pem_path)
        .map_err(|e| format!("leitura {}: {e}", pem_path.display()))?;
    let client_identity_pkcs12 = std::fs::read(&pfx_path)
        .map_err(|e| format!("leitura {}: {e}", pfx_path.display()))?;
    println!("PEM carregado de {}", pem_path.display());
    println!("PFX carregado de {} ({} bytes)", pfx_path.display(), client_identity_pkcs12.len());

    let client = SeriesClient::new(ClientConfig {
        endpoint: "https://servicos.portaldasfinancas.gov.pt:722/SeriesWSService".to_string(),
        // Override via env (`AT_USERNAME` / `AT_PASSWORD`) ao iterar credenciais.
        // Default: NIF/sub-user e password de teste documentados pela AT
        // (info.portaldasfinancas.gov.pt). Em data desta iteração ambas
        // estavam a falhar com faultcode 118 — provavelmente carecem de
        // permissão WSE activa no sub-utilizador 0037.
        username: std::env::var("AT_USERNAME")
            .unwrap_or_else(|_| "599999993/0037".to_string()),
        password: std::env::var("AT_PASSWORD")
            .unwrap_or_else(|_| "testes1234".to_string()),
        public_key_pem,
        client_identity_pkcs12,
        client_identity_password: "TESTEwebservice".to_string(),
        timeout: None,
    })?;

    println!("\n=== 1) consultarSeries (read-only) ===");
    match client.consultar_series(&ConsultarSeriesRequest::default()).await {
        Ok(list) => {
            println!("OK — {} séries devolvidas pela AT", list.len());
            for s in list.iter().take(5) {
                println!(
                    "  • {} ({}/{}) cod={} estado={} numIni={}",
                    s.serie, s.classe_doc, s.tipo_doc, s.cod_validacao_serie, s.estado, s.num_inicial_seq
                );
            }
            if list.len() > 5 {
                println!("  ... +{} séries", list.len() - 5);
            }
        }
        Err(e) => {
            println!("FALHA: {e}");
            // Imprime causas em cascata (e.g., timeout / connect refused / tls)
            let mut src: Option<&dyn std::error::Error> = Some(&e);
            while let Some(s) = src.and_then(|x| x.source()) {
                println!("  causa: {s}");
                src = Some(s);
            }
        }
    }

    println!("\n=== 2) registarSerie (cria série única) ===");
    // Prefixo único para evitar colisão com registos prévios do NIF de teste.
    // A AT impõe maxLength=35 em `serie`.
    let ts = Utc::now().timestamp();
    let serie = format!("OR{}", ts);
    let now = Utc::now();
    let req = RegistarSerieRequest {
        serie: serie.clone(),
        tipo_serie: TipoSerie::Normal,
        classe_doc: "SI".into(),
        tipo_doc: "FS".into(),
        num_inicial_seq: 1,
        data_inicio_prev_utiliz: now.date_naive(),
        num_cert_sw_fatur: 0,
        meio_processamento: "PF".into(),
    };
    println!("a registar série '{}' (FS/{}/{})...", serie, now.year(), now.date_naive());
    match client.registar_serie(&req).await {
        Ok(info) => {
            println!("OK — codValidacaoSerie={} estado={} dataRegisto={}",
                info.cod_validacao_serie, info.estado, info.data_registo);
        }
        Err(e) => println!("FALHA: {e}"),
    }

    Ok(())
}
