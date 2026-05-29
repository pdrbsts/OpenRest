//! Codificador ESC/POS: documento renderizado (texto + marcadores de estilo)
//! → bytes para uma impressora térmica.
//!
//! Pega no resultado de [`crate::template::render_document_mode`] em
//! [`crate::template::RenderMode::Markers`] e produz a sequência de bytes:
//! inicialização, selecção de codepage, estilos (negrito/duplo/sublinhado),
//! corte de papel e abertura de gaveta.
//!
//! Codepage: o texto é codificado em **Windows-1252**, que cobre todos os
//! acentos do português; o byte `n` de `ESC t n` (página do hardware) é
//! configurável por impressora — o default (16) é o WPC1252 em muitas Epson.
//! O envio dos bytes ao hardware é tratado por [`crate::transport`].

use encoding_rs::WINDOWS_1252;

use crate::template::markers;

/// Perfil ESC/POS de uma impressora (subconjunto do driver da spec §2.2).
#[derive(Debug, Clone)]
pub struct EscposProfile {
    /// Argumento `n` de `ESC t n` — página de caracteres do hardware para a
    /// codificação Windows-1252.
    pub codepage_byte: u8,
    /// Corta o papel no fim do documento.
    pub cut: bool,
    /// Envia o impulso de abertura de gaveta no início.
    pub open_drawer: bool,
    /// Linhas em branco antes do corte.
    pub feed_before_cut: u8,
}

impl Default for EscposProfile {
    fn default() -> Self {
        Self {
            codepage_byte: 16, // WPC1252
            cut: true,
            open_drawer: false,
            feed_before_cut: 3,
        }
    }
}

// Sequências ESC/POS.
const INIT: &[u8] = &[0x1B, 0x40]; // ESC @
const BOLD_ON: &[u8] = &[0x1B, 0x45, 0x01]; // ESC E 1
const BOLD_OFF: &[u8] = &[0x1B, 0x45, 0x00];
const UNDERLINE_ON: &[u8] = &[0x1B, 0x2D, 0x01]; // ESC - 1
const UNDERLINE_OFF: &[u8] = &[0x1B, 0x2D, 0x00];
const DOUBLE_ON: &[u8] = &[0x1D, 0x21, 0x11]; // GS ! 0x11 (duplo h+w)
const DOUBLE_OFF: &[u8] = &[0x1D, 0x21, 0x00];
const PARTIAL_CUT: &[u8] = &[0x1D, 0x56, 0x42, 0x00]; // GS V 66 0
/// ESC p 0 25*2ms 250*2ms — impulso na gaveta ligada ao pino 2.
const DRAWER_KICK: &[u8] = &[0x1B, 0x70, 0x00, 0x19, 0xFA];

/// Codifica um documento (com marcadores) em bytes ESC/POS.
pub fn encode(text_with_markers: &str, profile: &EscposProfile) -> Vec<u8> {
    let mut out = Vec::with_capacity(text_with_markers.len() + 32);
    out.extend_from_slice(INIT);
    out.extend_from_slice(&[0x1B, 0x74, profile.codepage_byte]); // ESC t n
    if profile.open_drawer {
        out.extend_from_slice(DRAWER_KICK);
    }

    let mut buf = String::new();
    let flush = |buf: &mut String, out: &mut Vec<u8>| {
        if !buf.is_empty() {
            let (encoded, _, _) = WINDOWS_1252.encode(buf);
            out.extend_from_slice(&encoded);
            buf.clear();
        }
    };

    for c in text_with_markers.chars() {
        match c {
            markers::RED_ON => {
                flush(&mut buf, &mut out);
                out.extend_from_slice(BOLD_ON);
            }
            markers::RED_OFF => {
                flush(&mut buf, &mut out);
                out.extend_from_slice(BOLD_OFF);
            }
            markers::DOUBLE_ON => {
                flush(&mut buf, &mut out);
                out.extend_from_slice(DOUBLE_ON);
            }
            markers::DOUBLE_OFF => {
                flush(&mut buf, &mut out);
                out.extend_from_slice(DOUBLE_OFF);
            }
            markers::UNDER_ON => {
                flush(&mut buf, &mut out);
                out.extend_from_slice(UNDERLINE_ON);
            }
            markers::UNDER_OFF => {
                flush(&mut buf, &mut out);
                out.extend_from_slice(UNDERLINE_OFF);
            }
            '\n' => {
                flush(&mut buf, &mut out);
                out.push(b'\n');
            }
            _ => buf.push(c),
        }
    }
    flush(&mut buf, &mut out);

    for _ in 0..profile.feed_before_cut {
        out.push(b'\n');
    }
    if profile.cut {
        out.extend_from_slice(PARTIAL_CUT);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::markers;

    #[test]
    fn starts_with_init_and_codepage() {
        let out = encode("ola", &EscposProfile::default());
        assert_eq!(&out[0..2], &[0x1B, 0x40]); // ESC @
        assert_eq!(&out[2..5], &[0x1B, 0x74, 16]); // ESC t 16
    }

    #[test]
    fn encodes_accents_in_1252() {
        let profile = EscposProfile { cut: false, feed_before_cut: 0, ..Default::default() };
        let out = encode("ção", &profile);
        // ç=0xE7, ã=0xE3, o=0x6F (1252).
        let tail = &out[5..]; // após init + codepage
        assert_eq!(tail, &[0xE7, 0xE3, 0x6F]);
    }

    #[test]
    fn translates_style_markers() {
        let profile = EscposProfile { cut: false, feed_before_cut: 0, ..Default::default() };
        let s = format!("{}X{}", markers::DOUBLE_ON, markers::DOUBLE_OFF);
        let out = encode(&s, &profile);
        let body = &out[5..];
        // GS ! 0x11, 'X', GS ! 0x00
        assert_eq!(body, &[0x1D, 0x21, 0x11, b'X', 0x1D, 0x21, 0x00]);
    }

    #[test]
    fn cut_and_drawer() {
        let profile = EscposProfile {
            cut: true,
            open_drawer: true,
            feed_before_cut: 2,
            ..Default::default()
        };
        let out = encode("x", &profile);
        // gaveta logo após o codepage
        assert_eq!(&out[5..10], DRAWER_KICK);
        // termina com 2 LFs + corte parcial
        assert!(out.ends_with(&[b'\n', b'\n', 0x1D, 0x56, 0x42, 0x00]));
    }
}
