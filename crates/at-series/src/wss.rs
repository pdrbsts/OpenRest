//! WS-Security UsernameToken para a AT — esquema oficial confirmado contra um
//! envelope real capturado em
//! `https://faturas.portaldasfinancas.gov.pt/testarLigacaoWebService.action`.
//!
//! ## Layout do `<wss:Security>`
//!
//! ```xml
//! <wss:Security xmlns:at="http://at.pt/wsp/auth"
//!               xmlns:wss="http://schemas.xmlsoap.org/ws/2002/12/secext"
//!               S:Actor="http://at.pt/actor/SPA"
//!               at:Version="2">
//!   <wss:UsernameToken>
//!     <wss:Username>{nif}/{subutilizador}</wss:Username>
//!     <wss:Password Digest="{digest_b64}">{ciphered_password_b64}</wss:Password>
//!     <wss:Nonce>{rsa_encrypted_nonce_b64}</wss:Nonce>
//!     <wss:Created>{utc_iso8601_with_ms}</wss:Created>
//!   </wss:UsernameToken>
//! </wss:Security>
//! ```
//!
//! ## Cifras
//!
//! 1. **Nonce raw**: 16 bytes aleatórios → também usado como chave AES-128.
//! 2. **`<wss:Nonce>`**: `Base64(RSA/PKCS1v15(nonce_raw))` usando a chave
//!    pública da AT. Comprimento = `key_size_bytes` (256 para RSA-2048, 512
//!    para RSA-4096). A AT validou que o ambiente actual usa RSA-4096.
//! 3. **`<wss:Password>` conteúdo**: `Base64(AES-128/ECB/PKCS7(password_plain))`
//!    com chave = nonce_raw.
//! 4. **`Digest` atributo**: `Base64(SHA-256(nonce_raw || created_text ||
//!    password_plain))`. Equivalente ao PasswordDigest do WSS standard mas
//!    com SHA-256 (32 bytes → 44 chars b64) em vez de SHA-1 (20 bytes).
//! 5. **`<wss:Created>`**: **texto plano**, formato ISO 8601 UTC com
//!    milissegundos (`%Y-%m-%dT%H:%M:%S%.3fZ`).
//!
//! ## Ordem dos elementos
//!
//! `Username → Password → Nonce → Created`. Validado contra envelope real.

use aes::Aes128;
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use ecb::Encryptor;
use rand::RngCore;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use sha2::{Digest, Sha256};

type Aes128EcbEnc = Encryptor<Aes128>;

const NONCE_LEN: usize = 16;

/// Constrói o XML do cabeçalho `<wss:Security>` pronto a injectar no
/// envelope SOAP.
pub fn build_security_header(
    username: &str,
    password: &str,
    at_public_key: &RsaPublicKey,
) -> Result<String, String> {
    // 1. Nonce aleatório (16 bytes — chave AES-128).
    let mut nonce = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce);

    // 2. Created — texto plano, UTC com milissegundos.
    let created = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    // 3. Password cifrada com AES/ECB/PKCS7 com o nonce como chave.
    let password_cipher = aes_ecb_pkcs7(&nonce, password.as_bytes())
        .map_err(|e| format!("AES password: {e}"))?;
    let password_b64 = general_purpose::STANDARD.encode(&password_cipher);

    // 4. Nonce cifrado com RSA/PKCS1v15 com a chave pública da AT.
    let mut rng = rand::thread_rng();
    let nonce_cipher = at_public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &nonce)
        .map_err(|e| format!("RSA nonce: {e}"))?;
    // A AT formata o Nonce em base64 com quebras de linha de 76 chars (MIME)
    // — não estritamente necessário porque o XML aceita base64 sem quebras,
    // mas tentamos espelhar o envelope oficial para o caso de o parser do
    // backend ser sensível ao formato.
    let nonce_b64 = mime_base64(&nonce_cipher);

    // 5. Digest = Base64(SHA-256(nonce_raw || created_text || password_plain)).
    let mut hasher = Sha256::new();
    hasher.update(nonce);
    hasher.update(created.as_bytes());
    hasher.update(password.as_bytes());
    let digest_b64 = general_purpose::STANDARD.encode(hasher.finalize());

    // Envelope: namespaces e atributos espelhados de um envelope real da AT.
    let xml = format!(
        concat!(
            "<wss:Security xmlns:at=\"http://at.pt/wsp/auth\" ",
            "xmlns:wss=\"http://schemas.xmlsoap.org/ws/2002/12/secext\" ",
            "S:Actor=\"http://at.pt/actor/SPA\" at:Version=\"2\">",
            "<wss:UsernameToken>",
            "<wss:Username>{user}</wss:Username>",
            "<wss:Password Digest=\"{digest}\">{pwd}</wss:Password>",
            "<wss:Nonce>{nonce}</wss:Nonce>",
            "<wss:Created>{created}</wss:Created>",
            "</wss:UsernameToken>",
            "</wss:Security>"
        ),
        user = xml_escape(username),
        digest = digest_b64,
        pwd = password_b64,
        nonce = nonce_b64,
        created = created,
    );
    Ok(xml)
}

fn aes_ecb_pkcs7(key: &[u8], data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if key.len() != 16 {
        return Err("AES key must be 16 bytes");
    }
    let cipher = Aes128EcbEnc::new_from_slice(key).map_err(|_| "AES init")?;
    let mut buf = vec![0u8; data.len() + 16];
    buf[..data.len()].copy_from_slice(data);
    let ciphertext_len = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buf, data.len())
        .map_err(|_| "AES encrypt")?
        .len();
    buf.truncate(ciphertext_len);
    Ok(buf)
}

/// Base64 estilo MIME (quebras de linha a cada 76 chars) — formato que a AT
/// usa no `<wss:Nonce>` real.
fn mime_base64(data: &[u8]) -> String {
    let raw = general_purpose::STANDARD.encode(data);
    let mut out = String::with_capacity(raw.len() + raw.len() / 76);
    let chars: Vec<char> = raw.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && i % 76 == 0 {
            out.push('\n');
        }
        out.push(*c);
    }
    out
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes_ecb_pkcs7_pads_full_block() {
        let key = [0u8; 16];
        let out = aes_ecb_pkcs7(&key, b"hello").unwrap();
        assert_eq!(out.len(), 16);
    }

    #[test]
    fn aes_ecb_pkcs7_two_blocks() {
        let key = [1u8; 16];
        let out = aes_ecb_pkcs7(&key, b"this is sixteen!").unwrap();
        assert_eq!(out.len(), 32);
    }

    #[test]
    fn xml_escape_escapes_specials() {
        assert_eq!(xml_escape("a<b&c\">"), "a&lt;b&amp;c&quot;&gt;");
    }

    #[test]
    fn mime_base64_breaks_at_76() {
        // 100 bytes => 136 chars base64 + padding → uma linha de 76 + outra.
        let data = vec![0u8; 100];
        let out = mime_base64(&data);
        let first_line = out.lines().next().unwrap();
        assert_eq!(first_line.chars().count(), 76);
    }
}
