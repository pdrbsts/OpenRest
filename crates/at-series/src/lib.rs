//! Cliente do web-service `SeriesWS` da AT (Autoridade Tributária) para
//! comunicação de séries de facturação. Implementa as 4 operações definidas
//! pela WSDL (`series.wsdl`):
//!
//! * `registarSerie` — comunica uma nova série e obtém `codValidacaoSerie`.
//! * `consultarSeries` — pesquisa séries previamente comunicadas.
//! * `finalizarSerie` — encerra uma série com indicação do último doc emitido.
//! * `anularSerie` — anula uma comunicação anteriormente feita (apenas se
//!   nenhum documento foi ainda emitido com essa série).
//!
//! ## Autenticação
//!
//! A AT exige WS-Security UsernameToken com cifra híbrida específica:
//!
//! 1. `Nonce` = 16 bytes aleatórios encriptados com `RSA/None/PKCS1Padding`
//!    usando a chave pública da AT (do ambiente respectivo).
//! 2. `Password` = texto plano da password do utilizador (NIF/subuser)
//!    encriptado com `AES/ECB/PKCS5Padding` usando o Nonce como chave.
//! 3. `Created` = timestamp ISO 8601 em milissegundos, igualmente cifrado
//!    com AES/ECB/PKCS5Padding sobre o Nonce.
//!
//! Todos os três campos são depois codificados em base64.
//!
//! ## Ambientes
//!
//! - **Teste**: `https://servicos.portaldasfinancas.gov.pt:722/SeriesWSService`
//!   com NIF `599999993` (sub-utilizador `0037` por defeito).
//! - **Produção**: depende do certificado/credencial atribuído pela AT ao
//!   sujeito passivo.

pub mod soap;
pub mod types;
pub mod wss;

use rsa::RsaPublicKey;
use std::time::Duration;

pub use types::*;

#[derive(Debug, thiserror::Error)]
pub enum AtError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("WS-Security: {0}")]
    Security(String),
    #[error("SOAP parsing: {0}")]
    Parse(String),
    #[error("AT returned error code {code}: {msg}")]
    AtFault { code: i32, msg: String },
    #[error("Invalid configuration: {0}")]
    Config(String),
}

/// Cliente do SeriesWS da AT. Reutilizável (segura threadsafe), guarda a
/// chave pública parseada e o cliente HTTP. Cada chamada cria seu próprio
/// nonce/timestamp.
pub struct SeriesClient {
    http: reqwest::Client,
    endpoint: String,
    username: String,
    password: String,
    public_key: RsaPublicKey,
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// URL completa do endpoint, e.g.
    /// `https://servicos.portaldasfinancas.gov.pt:722/SeriesWSService`.
    pub endpoint: String,
    /// `NIF/subutilizador`, e.g. `599999993/0037`.
    pub username: String,
    /// Password em texto plano. É encriptada ANTES de ser enviada.
    pub password: String,
    /// Chave pública RSA da AT (PEM ou DER) para cifrar o nonce.
    pub public_key_pem: String,
    /// Identidade mTLS — PFX/PKCS#12 binário com a chave + certificado de
    /// cliente emitido pela AT. Obrigatório: o endpoint `:722`/`:422` rejeita
    /// ligações sem certificado de cliente válido (aceita "AT Issuing CA1"
    /// ou "DGITA Issuing CA1").
    pub client_identity_pkcs12: Vec<u8>,
    /// Password do PFX. Default do ficheiro de testes: `TESTEwebservice`.
    pub client_identity_password: String,
    /// Timeout das chamadas HTTP. Default 30s.
    pub timeout: Option<Duration>,
}

impl SeriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, AtError> {
        use rsa::pkcs8::DecodePublicKey;
        let public_key = RsaPublicKey::from_public_key_pem(&config.public_key_pem)
            .or_else(|_| {
                // Algumas distribuições da AT fornecem a chave em formato
                // PKCS#1 puro (RSAPublicKey). Tentar ambas variantes.
                use rsa::pkcs1::DecodeRsaPublicKey;
                RsaPublicKey::from_pkcs1_pem(&config.public_key_pem)
            })
            .map_err(|e| AtError::Config(format!("invalid AT public key: {e}")))?;
        let identity = reqwest::Identity::from_pkcs12_der(
            &config.client_identity_pkcs12,
            &config.client_identity_password,
        )
        .map_err(|e| AtError::Config(format!("client identity PFX: {e}")))?;
        let http = reqwest::Client::builder()
            .timeout(config.timeout.unwrap_or(Duration::from_secs(30)))
            .identity(identity)
            // O endpoint :722 da AT serve HTTP/1.1 apenas — desactivamos
            // explicitamente HTTP/2 para evitar negociações ALPN incompatíveis.
            .http1_only()
            .build()
            .map_err(AtError::Http)?;
        Ok(Self {
            http,
            endpoint: config.endpoint,
            username: config.username,
            password: config.password,
            public_key,
        })
    }

    /// Comunica uma nova série à AT. Devolve o `seriesInfo` retornado, do
    /// qual o campo mais relevante é `codValidacaoSerie` (8 chars, usado na
    /// construção do ATCUD).
    pub async fn registar_serie(
        &self,
        req: &RegistarSerieRequest,
    ) -> Result<SeriesInfo, AtError> {
        let body = soap::build_registar_serie(req);
        let envelope = self.wrap_envelope("registarSerie", &body)?;
        let response = self.post(&envelope, "registarSerieRequest").await?;
        soap::parse_series_resp(&response, "registarSerieResp")
    }

    pub async fn consultar_series(
        &self,
        req: &ConsultarSeriesRequest,
    ) -> Result<Vec<SeriesInfo>, AtError> {
        let body = soap::build_consultar_series(req);
        let envelope = self.wrap_envelope("consultarSeries", &body)?;
        let response = self.post(&envelope, "consultarSeriesRequest").await?;
        soap::parse_consult_resp(&response)
    }

    pub async fn finalizar_serie(
        &self,
        req: &FinalizarSerieRequest,
    ) -> Result<SeriesInfo, AtError> {
        let body = soap::build_finalizar_serie(req);
        let envelope = self.wrap_envelope("finalizarSerie", &body)?;
        let response = self.post(&envelope, "finalizarSerieRequest").await?;
        soap::parse_series_resp(&response, "finalizarSerieResp")
    }

    pub async fn anular_serie(
        &self,
        req: &AnularSerieRequest,
    ) -> Result<SeriesInfo, AtError> {
        let body = soap::build_anular_serie(req);
        let envelope = self.wrap_envelope("anularSerie", &body)?;
        let response = self.post(&envelope, "anularSerieRequest").await?;
        soap::parse_series_resp(&response, "anularSerieResp")
    }

    fn wrap_envelope(&self, _op_name: &str, body_xml: &str) -> Result<String, AtError> {
        let header = wss::build_security_header(&self.username, &self.password, &self.public_key)
            .map_err(AtError::Security)?;
        Ok(soap::wrap_envelope(&header, body_xml))
    }

    async fn post(&self, envelope: &str, soap_action: &str) -> Result<String, AtError> {
        // Debug opcional (útil para diagnosticar falhas WS-Security): activar
        // com `AT_DEBUG=1`. Logamos request e response em stderr.
        let debug = std::env::var("AT_DEBUG").ok().as_deref() == Some("1");
        if debug {
            eprintln!("\n--- REQUEST ({}) ---\n{}\n", soap_action, envelope);
        }
        let response = self
            .http
            .post(&self.endpoint)
            .header("Content-Type", "text/xml; charset=utf-8")
            .header("SOAPAction", format!("\"http://at.gov.pt/SeriesWS/{}\"", soap_action))
            .body(envelope.to_string())
            .send()
            .await?;
        let status = response.status();
        let text = response.text().await?;
        if debug {
            eprintln!("--- RESPONSE ({}) ---\n{}\n", status, text);
        }
        Ok(text)
    }
}
