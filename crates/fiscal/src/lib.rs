//! Fiscal compliance helpers (Portugal Phase 1):
//! - Signing key load/generate (RSA-2048).
//! - Document signing (Portaria 363/2010 — PKCS#1 v1.5 SHA-1).
//! - ATCUD composition.
//! - QR Code payload (Portaria 195/2020) + bitmap render for receipts.

use std::path::Path;

use base64::Engine;
use chrono::{DateTime, Utc};
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, LineEnding};
use rsa::pkcs1v15::SigningKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::RsaPrivateKey;
use sha1::Sha1;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FiscalError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("rsa: {0}")]
    Rsa(#[from] rsa::errors::Error),
    #[error("pkcs1: {0}")]
    Pkcs1(#[from] rsa::pkcs1::Error),
}

/// Loads the signing key from `path`. Creates and persists a fresh 2048-bit
/// RSA key on first run.
pub fn load_or_generate_key(path: &Path) -> Result<RsaPrivateKey, FiscalError> {
    if path.exists() {
        let pem = std::fs::read_to_string(path)?;
        return Ok(RsaPrivateKey::from_pkcs1_pem(&pem).map_err(|e| FiscalError::Pkcs1(e))?);
    }
    let mut rng = rand::thread_rng();
    let key = RsaPrivateKey::new(&mut rng, 2048)?;
    let pem = key.to_pkcs1_pem(LineEnding::LF).map_err(|e| FiscalError::Pkcs1(e))?;
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }
    std::fs::write(path, pem.as_bytes())?;
    Ok(key)
}

/// Content signed for each document, per Portaria 363/2010:
///   YYYY-MM-DD;YYYY-MM-DDThh:mm:ss;<DocID>;<Total>;<HashAnterior>
pub fn signing_payload(
    document_date: DateTime<Utc>,
    issued_at: DateTime<Utc>,
    doc_identifier: &str,
    total_cents: i64,
    previous_hash_base64: &str,
) -> String {
    let total = format!("{}.{:02}", total_cents / 100, (total_cents.abs()) % 100);
    format!(
        "{};{};{};{};{}",
        document_date.format("%Y-%m-%d"),
        issued_at.format("%Y-%m-%dT%H:%M:%S"),
        doc_identifier,
        total,
        previous_hash_base64
    )
}

/// Signs `payload` with PKCS#1 v1.5 SHA-1 and returns the signature encoded as
/// base64. The 4-character "Q" identifier (positions 1, 11, 21, 31, 1-indexed)
/// is also returned.
pub fn sign(key: &RsaPrivateKey, payload: &str) -> (String, String) {
    let signing_key = SigningKey::<Sha1>::new(key.clone());
    let mut rng = rand::thread_rng();
    let sig = signing_key.sign_with_rng(&mut rng, payload.as_bytes());
    let b64 = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes());
    let q = q_chars(&b64);
    (b64, q)
}

fn q_chars(b64: &str) -> String {
    let bytes = b64.as_bytes();
    let pick = |pos1: usize| -> char {
        bytes
            .get(pos1.saturating_sub(1))
            .map(|b| *b as char)
            .unwrap_or('?')
    };
    [pick(1), pick(11), pick(21), pick(31)].iter().collect()
}

/// Builds the ATCUD identifier: `validation-number`, e.g. `JFG3-12345`.
pub fn atcud(validation_code: &str, document_number: i32) -> String {
    format!("{}-{}", validation_code, document_number)
}

/// Valida o check-digit de um NIF português (9 dígitos, módulo 11).
/// Primeiro dígito tem de pertencer ao conjunto {1,2,3,5,6,7,8,9} (singulares,
/// colectivos, eventuais). Spec §201: a UI deve avisar mas não impedir, daí
/// devolvermos `bool` em vez de `Result`.
pub fn validate_nif_pt(nif: &str) -> bool {
    let digits: Vec<u32> = nif.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 9 {
        return false;
    }
    if !matches!(digits[0], 1 | 2 | 3 | 5 | 6 | 7 | 8 | 9) {
        return false;
    }
    let sum: u32 = digits[..8]
        .iter()
        .enumerate()
        .map(|(i, d)| d * (9 - i as u32))
        .sum();
    let check = match 11 - (sum % 11) {
        10 | 11 => 0,
        v => v,
    };
    check == digits[8]
}

#[cfg(test)]
mod nif_tests {
    use super::*;

    #[test]
    fn known_valid_pt_nifs() {
        // NIFs sintéticos com check-digit correcto.
        assert!(validate_nif_pt("123456789"));
        assert!(validate_nif_pt("500000000"));
    }

    #[test]
    fn rejects_short_or_letters() {
        assert!(!validate_nif_pt("12345678"));
        assert!(!validate_nif_pt("12345678X"));
        assert!(!validate_nif_pt(""));
    }

    #[test]
    fn rejects_wrong_check_digit() {
        assert!(!validate_nif_pt("123456788"));
    }

    #[test]
    fn rejects_invalid_first_digit() {
        // 4 não é prefixo válido em NIF singular/colectivo PT.
        assert!(!validate_nif_pt("400000004"));
    }
}

/// Inputs to build the QR payload required by Portaria 195/2020.
pub struct QrInputs<'a> {
    pub emitter_nif: &'a str,
    pub customer_nif: &'a str,
    pub country: &'a str,
    pub document_type: &'a str,
    pub document_status: &'a str,
    pub document_date: DateTime<Utc>,
    pub document_identifier: &'a str,
    pub atcud: &'a str,
    pub tax_country: &'a str,
    /// Per-rate breakdown — `(rate_label, base_cents, vat_cents)`.
    /// Phase 1 only emits the I7/I8 (23%) or I5/I6 (13%) pair we have.
    pub vat_breakdown: &'a [(VatRate, i64, i64)],
    pub total_vat_cents: i64,
    pub total_with_vat_cents: i64,
    pub hash_short: &'a str,
    pub software_certificate: &'a str,
}

#[derive(Copy, Clone, Debug)]
pub enum VatRate {
    Exempt,
    Reduced,
    Intermediate,
    Standard,
}

impl VatRate {
    pub fn from_basis_points(bp: i32) -> Self {
        match bp {
            0 => VatRate::Exempt,
            1 ..= 700 => VatRate::Reduced,
            701 ..= 1500 => VatRate::Intermediate,
            _ => VatRate::Standard,
        }
    }

    pub fn percent_label(&self) -> &'static str {
        match self {
            VatRate::Exempt => "ISE",
            VatRate::Reduced => "6%",
            VatRate::Intermediate => "13%",
            VatRate::Standard => "23%",
        }
    }
}

fn cents_str(v: i64) -> String {
    format!("{}.{:02}", v / 100, v.abs() % 100)
}

pub fn qr_payload(inputs: &QrInputs<'_>) -> String {
    let mut parts: Vec<String> = Vec::new();
    parts.push(format!("A:{}", inputs.emitter_nif));
    parts.push(format!("B:{}", inputs.customer_nif));
    parts.push(format!("C:{}", inputs.country));
    parts.push(format!("D:{}", inputs.document_type));
    parts.push(format!("E:{}", inputs.document_status));
    parts.push(format!("F:{}", inputs.document_date.format("%Y%m%d")));
    parts.push(format!("G:{}", inputs.document_identifier));
    parts.push(format!("H:{}", inputs.atcud));
    parts.push(format!("I1:{}", inputs.tax_country));

    let mut exempt = 0_i64;
    let mut b6 = (0_i64, 0_i64);
    let mut b13 = (0_i64, 0_i64);
    let mut b23 = (0_i64, 0_i64);
    for (rate, base, vat) in inputs.vat_breakdown {
        match rate {
            VatRate::Exempt => exempt += base,
            VatRate::Reduced => {
                b6.0 += base;
                b6.1 += vat;
            }
            VatRate::Intermediate => {
                b13.0 += base;
                b13.1 += vat;
            }
            VatRate::Standard => {
                b23.0 += base;
                b23.1 += vat;
            }
        }
    }
    if exempt > 0 {
        parts.push(format!("I2:{}", cents_str(exempt)));
    }
    if b6.0 > 0 {
        parts.push(format!("I3:{}", cents_str(b6.0)));
        parts.push(format!("I4:{}", cents_str(b6.1)));
    }
    if b13.0 > 0 {
        parts.push(format!("I5:{}", cents_str(b13.0)));
        parts.push(format!("I6:{}", cents_str(b13.1)));
    }
    if b23.0 > 0 {
        parts.push(format!("I7:{}", cents_str(b23.0)));
        parts.push(format!("I8:{}", cents_str(b23.1)));
    }
    parts.push(format!("N:{}", cents_str(inputs.total_vat_cents)));
    parts.push(format!("O:{}", cents_str(inputs.total_with_vat_cents)));
    parts.push(format!("Q:{}", inputs.hash_short));
    parts.push(format!("R:{}", inputs.software_certificate));

    parts.join("*")
}

/// Renders `data` as a QR Code using half-block characters so it fits in a
/// monospace text receipt. Each output row collapses two QR rows so the code
/// stays roughly square at 1-cell horizontal pitch.
pub fn render_qr_ascii(data: &str) -> Result<String, qrcode::types::QrError> {
    use qrcode::{EcLevel, QrCode};
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::M)?;
    let width = code.width();
    let modules: Vec<bool> = code.to_colors().into_iter().map(|c| c == qrcode::Color::Dark).collect();
    let module = |x: usize, y: usize| -> bool {
        if x >= width || y >= width {
            false
        } else {
            modules[y * width + x]
        }
    };

    let quiet = 2;
    let mut out = String::new();
    let total = width + quiet * 2;
    let mut y = 0;
    while y < total {
        for x in 0..total {
            let qx = x as isize - quiet as isize;
            let ux = y as isize - quiet as isize;
            let lx = (y + 1) as isize - quiet as isize;
            let upper = if qx < 0 || ux < 0 || qx as usize >= width || ux as usize >= width {
                false
            } else {
                module(qx as usize, ux as usize)
            };
            let lower = if qx < 0 || lx < 0 || qx as usize >= width || lx as usize >= width {
                false
            } else {
                module(qx as usize, lx as usize)
            };
            let ch = match (upper, lower) {
                (true, true) => '\u{2588}',  // FULL BLOCK
                (true, false) => '\u{2580}', // UPPER HALF BLOCK
                (false, true) => '\u{2584}', // LOWER HALF BLOCK
                (false, false) => ' ',
            };
            out.push(ch);
        }
        out.push('\n');
        y += 2;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_chars_picks_correct_positions() {
        // Portaria 195/2020 (anexo do QR Code, campo I) e spec interna
        // 05-integrations/02-fiscal-compliance.md §3.2: caracteres das
        // posições 1, 11, 21 e 31 (1-indexed) da assinatura Base64.
        let s = "abcdefghijklmnopqrstuvwxyzABCDEFG";
        assert_eq!(q_chars(s), "akuE");
    }

    #[test]
    fn signing_payload_format() {
        let date = DateTime::parse_from_rfc3339("2026-05-27T08:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let p = signing_payload(date, date, "FS A/1", 450, "prevhash");
        assert_eq!(p, "2026-05-27;2026-05-27T08:00:00;FS A/1;4.50;prevhash");
    }
}
