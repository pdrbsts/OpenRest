//! Impressão em A4 para impressoras "normais" instaladas no Windows (laser/jato),
//! para documentos como a fatura a crédito.
//!
//! Recebe texto monoespaçado já paginado em colunas (e.g., o resultado do motor
//! de templates) e produz:
//! - [`to_pdf`] — PDF A4 (Courier), cross-platform, para arquivo/email.
//! - [`print_gdi`] — desenho via GDI numa impressora instalada no Windows.
//!
//! Usar fonte monoespaçada (Courier) deixa o alinhamento por colunas (já feito
//! pelo template) funcionar tal e qual no papel, sem métricas de fonte.

use crate::PrinterError;

/// Estilo de página A4 monoespaçada.
#[derive(Debug, Clone)]
pub struct A4Style {
    pub font_size_pt: f32,
    pub line_height_pt: f32,
    pub margin_mm: f32,
}

impl Default for A4Style {
    fn default() -> Self {
        Self {
            font_size_pt: 10.0,
            line_height_pt: 12.0,
            margin_mm: 15.0,
        }
    }
}

const A4_WIDTH_MM: f32 = 210.0;
const A4_HEIGHT_MM: f32 = 297.0;
const PT_TO_MM: f32 = 0.352_778;

/// Gera um PDF A4 a partir do texto monoespaçado. Pagina automaticamente.
pub fn to_pdf(text: &str, style: &A4Style) -> Result<Vec<u8>, PrinterError> {
    use printpdf::{BuiltinFont, Mm, PdfDocument};

    let (doc, page1, layer1) =
        PdfDocument::new("Fatura", Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
    let font = doc
        .add_builtin_font(BuiltinFont::Courier)
        .map_err(|e| PrinterError::Connection(format!("pdf font: {e}")))?;

    let lh_mm = style.line_height_pt * PT_TO_MM;
    let top = A4_HEIGHT_MM - style.margin_mm;
    let bottom = style.margin_mm;

    let mut layer = doc.get_page(page1).get_layer(layer1);
    let mut y = top;
    for line in text.split('\n') {
        if y - lh_mm < bottom {
            let (p, l) = doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
            layer = doc.get_page(p).get_layer(l);
            y = top;
        }
        y -= lh_mm;
        // Linhas vazias apenas avançam o cursor.
        if !line.trim().is_empty() {
            layer.use_text(line, style.font_size_pt, Mm(style.margin_mm), Mm(y), &font);
        }
    }

    let mut bytes = Vec::new();
    {
        let mut writer = std::io::BufWriter::new(&mut bytes);
        doc.save(&mut writer)
            .map_err(|e| PrinterError::Connection(format!("pdf save: {e}")))?;
    }
    Ok(bytes)
}

/// Imprime o texto monoespaçado numa impressora instalada no Windows, via GDI.
#[cfg(windows)]
pub fn print_gdi(printer_name: &str, text: &str, style: &A4Style) -> Result<(), PrinterError> {
    gdi::print(printer_name, text, style)
}

#[cfg(not(windows))]
pub fn print_gdi(_printer_name: &str, _text: &str, _style: &A4Style) -> Result<(), PrinterError> {
    Err(PrinterError::Unsupported(
        "impressão GDI só está disponível no Windows".into(),
    ))
}

/// FFI mínimo ao GDI + winspool para desenhar texto numa impressora.
///
/// NOTA: este caminho não é testável sem uma impressora Windows real — deve ser
/// validado no local. A lógica de paginação e escala em DPI está aqui; os tipos
/// e assinaturas são os estáveis do Win32.
#[cfg(windows)]
mod gdi {
    use std::os::raw::c_void;

    use crate::PrinterError;

    use super::A4Style;

    type Handle = *mut c_void;
    type Bool = i32;

    #[repr(C)]
    struct DocInfoW {
        cb_size: i32,
        lpsz_doc_name: *const u16,
        lpsz_output: *const u16,
        lpsz_datatype: *const u16,
        fw_type: u32,
    }

    // Índices de GetDeviceCaps.
    const HORZRES: i32 = 8;
    const VERTRES: i32 = 10;
    const LOGPIXELSX: i32 = 88;
    const LOGPIXELSY: i32 = 90;

    const DEFAULT_CHARSET: u32 = 1;
    const FW_NORMAL: i32 = 400;

    #[link(name = "gdi32")]
    extern "system" {
        fn CreateDCW(driver: *const u16, device: *const u16, output: *const u16, init: *const c_void) -> Handle;
        fn DeleteDC(hdc: Handle) -> Bool;
        fn StartDocW(hdc: Handle, info: *const DocInfoW) -> i32;
        fn EndDoc(hdc: Handle) -> i32;
        fn StartPage(hdc: Handle) -> i32;
        fn EndPage(hdc: Handle) -> i32;
        fn GetDeviceCaps(hdc: Handle, index: i32) -> i32;
        #[allow(clippy::too_many_arguments)]
        fn CreateFontW(
            height: i32, width: i32, escapement: i32, orientation: i32, weight: i32,
            italic: u32, underline: u32, strikeout: u32, charset: u32, out_precision: u32,
            clip_precision: u32, quality: u32, pitch_and_family: u32, face: *const u16,
        ) -> Handle;
        fn SelectObject(hdc: Handle, obj: Handle) -> Handle;
        fn DeleteObject(obj: Handle) -> Bool;
        fn TextOutW(hdc: Handle, x: i32, y: i32, text: *const u16, len: i32) -> Bool;
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn print(printer_name: &str, text: &str, style: &A4Style) -> Result<(), PrinterError> {
        let name = wide(printer_name);
        let face = wide("Courier New");
        let doc_name = wide("OpenRest Fatura");
        unsafe {
            let hdc = CreateDCW(std::ptr::null(), name.as_ptr(), std::ptr::null(), std::ptr::null());
            if hdc.is_null() {
                return Err(PrinterError::Connection(format!(
                    "CreateDC falhou para '{printer_name}'"
                )));
            }
            let result = print_inner(hdc, text, style, &face, &doc_name);
            DeleteDC(hdc);
            result
        }
    }

    unsafe fn print_inner(
        hdc: Handle,
        text: &str,
        style: &A4Style,
        face: &[u16],
        doc_name: &[u16],
    ) -> Result<(), PrinterError> {
        let dpi_x = GetDeviceCaps(hdc, LOGPIXELSX).max(96);
        let dpi_y = GetDeviceCaps(hdc, LOGPIXELSY).max(96);
        let page_h = GetDeviceCaps(hdc, VERTRES);
        let _page_w = GetDeviceCaps(hdc, HORZRES);

        // pt → pixels do dispositivo.
        let font_px = (style.font_size_pt * dpi_y as f32 / 72.0).round() as i32;
        let line_px = (style.line_height_pt * dpi_y as f32 / 72.0).round() as i32;
        let margin_px_x = (style.margin_mm / 25.4 * dpi_x as f32).round() as i32;
        let margin_px_y = (style.margin_mm / 25.4 * dpi_y as f32).round() as i32;

        let hfont = CreateFontW(
            -font_px, 0, 0, 0, FW_NORMAL, 0, 0, 0, DEFAULT_CHARSET, 0, 0, 0, 0, face.as_ptr(),
        );
        let old_font = SelectObject(hdc, hfont);

        let info = DocInfoW {
            cb_size: std::mem::size_of::<DocInfoW>() as i32,
            lpsz_doc_name: doc_name.as_ptr(),
            lpsz_output: std::ptr::null(),
            lpsz_datatype: std::ptr::null(),
            fw_type: 0,
        };

        let mut err: Option<PrinterError> = None;
        if StartDocW(hdc, &info) <= 0 {
            err = Some(PrinterError::Connection("StartDoc falhou".into()));
        } else {
            StartPage(hdc);
            let mut y = margin_px_y;
            for line in text.split('\n') {
                if y + line_px > page_h - margin_px_y {
                    EndPage(hdc);
                    StartPage(hdc);
                    y = margin_px_y;
                }
                let wline = wide(line);
                // len em caracteres UTF-16 (sem o terminador nulo).
                TextOutW(hdc, margin_px_x, y, wline.as_ptr(), (wline.len() - 1) as i32);
                y += line_px;
            }
            EndPage(hdc);
            EndDoc(hdc);
        }

        SelectObject(hdc, old_font);
        DeleteObject(hfont);
        match err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdf_has_valid_header() {
        let text = "FACTURA\n\nArtigo            Total\nCafé               0.80\n";
        let bytes = to_pdf(text, &A4Style::default()).unwrap();
        assert!(bytes.starts_with(b"%PDF"), "não começa com %PDF");
        assert!(bytes.len() > 200, "PDF demasiado pequeno");
    }

    #[test]
    fn pdf_paginates_long_text() {
        // Muitas linhas forçam ≥2 páginas; não deve falhar.
        let mut text = String::new();
        for i in 0..200 {
            text.push_str(&format!("Linha {i}\n"));
        }
        let bytes = to_pdf(&text, &A4Style::default()).unwrap();
        assert!(bytes.starts_with(b"%PDF"));
    }

    #[cfg(not(windows))]
    #[test]
    fn gdi_unsupported_off_windows() {
        assert!(print_gdi("X", "y", &A4Style::default()).is_err());
    }
}
