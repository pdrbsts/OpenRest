//! Motor de documentos configuráveis (cabeçalhos/rodapés/detalhe com flags).
//!
//! Substitui as sequências especiais (`\no`, `\ds`, `\dt`, …) e a construção
//! XML-like `<! type="…" id="…" … !>` documentadas em
//! `spec/08-appendices/02-printer-flags.md` pelos valores correntes do
//! documento. Renderiza para texto simples alinhado a uma largura fixa (modo
//! impressora genérica file-based da Fase 2); as flags de estilo (`\s0`..`\s6`)
//! são reconhecidas mas não emitem sequências ESC neste modo — apenas as de
//! alinhamento (`\s7`/`\s8`/`\s9`) têm efeito visível.

use chrono::{DateTime, Utc};

pub const DEFAULT_WIDTH: usize = 48;

// ---------------------------------------------------------------------------
// Contexto
// ---------------------------------------------------------------------------

/// Dados do estabelecimento (origem: licença). Alimenta as flags `\no`..`\nc`.
#[derive(Default, Clone, Debug)]
pub struct Company {
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub nif: String,
    pub address: String,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub registry_office: Option<String>,
    pub registry_number: Option<String>,
    pub share_capital_cents: Option<i64>,
}

/// Cliente (e eventual associação). Alimenta as flags `\ol`..`\xz`.
#[derive(Default, Clone, Debug)]
pub struct Party {
    pub name: Option<String>,
    pub number: Option<String>,
    pub nif: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub zone: Option<String>,
    pub association_name: Option<String>,
    pub association_nif: Option<String>,
}

/// Empregado. Alimenta `\ne`/`\oe` e `memp_*`.
#[derive(Default, Clone, Debug)]
pub struct Staff {
    pub number: Option<String>,
    pub name: Option<String>,
}

/// Linha da tabela de IVA (rodapé, flag `\ti`).
#[derive(Clone, Debug)]
pub struct VatRow {
    pub label: String,
    pub base: i64,
    pub vat: i64,
}

/// Forma de pagamento aplicada ao documento.
#[derive(Default, Clone, Debug)]
pub struct PaymentLine {
    pub method: String,
    pub amount: i64,
}

/// Contexto de uma linha de detalhe (alimenta o template `linha_detalhe` e os
/// campos `fb_d_*`).
#[derive(Default, Clone, Debug)]
pub struct LineContext {
    /// Quantidade em milli-unidades (1000 = 1 unidade).
    pub qty_milli: i64,
    pub article_code: Option<String>,
    pub name: String,
    pub short_name: Option<String>,
    pub unit_price: i64,
    pub price_sem_iva: i64,
    /// Percentagem de desconto em pontos-base (1300 = 13%).
    pub perc_desc_bp: i32,
    pub val_desc: i64,
    pub iva_cod: Option<String>,
    /// Taxa de IVA em pontos-base (1300 = 13%).
    pub iva_perc_bp: i32,
    pub total: i64,
    pub zona_imp: Option<String>,
    pub emp_pedido: Option<String>,
    pub hora: Option<DateTime<Utc>>,
}

/// Reúne todos os valores substituíveis num documento.
#[derive(Default, Clone, Debug)]
pub struct DocumentContext {
    pub company: Company,
    pub client: Option<Party>,
    pub employee: Staff,
    pub table_number: Option<String>,
    pub table_name: Option<String>,
    pub local_name: Option<String>,
    pub issued_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub now: Option<DateTime<Utc>>,
    pub document_number: Option<String>,
    pub series: Option<String>,
    pub document_type_label: Option<String>,
    pub atcud: Option<String>,
    pub hash_short: Option<String>,
    pub software_version: Option<String>,
    pub software_certificate: Option<String>,
    pub num_people: Option<i64>,
    pub subtotal: i64,
    pub total: i64,
    pub total_sem_iva: i64,
    pub iva_total: i64,
    /// Factor de conversão para a moeda secundária (e.g. câmbio). `None`
    /// deixa as flags `\ve`/`\te`/`\pe` vazias.
    pub secondary_rate: Option<f64>,
    pub payments: Vec<PaymentLine>,
    pub troco: i64,
    pub gorjeta: i64,
    pub pago: i64,
    pub a1: Option<String>,
    pub a2: Option<String>,
    pub a3: Option<String>,
    pub lines: Vec<LineContext>,
    pub vat_rows: Vec<VatRow>,
    /// Bloco ASCII do QR Code (já renderizado). Vazio = não imprime arte.
    pub qr_block: String,
    pub qr_payload: String,
}

/// Template de um tipo de documento. As três secções partilham o mesmo motor
/// de flags; `linha_detalhe` é renderizada uma vez por cada linha do documento.
#[derive(Clone, Debug)]
pub struct DocumentTemplate {
    pub cabecalho: String,
    pub linha_detalhe: String,
    pub rodape: String,
    /// Imprime apenas o total (salta o bloco de detalhe).
    pub nao_imprime_detalhes: bool,
}

impl Default for DocumentTemplate {
    fn default() -> Self {
        Self {
            cabecalho: String::new(),
            linha_detalhe: String::new(),
            rodape: String::new(),
            nao_imprime_detalhes: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers de formatação
// ---------------------------------------------------------------------------

fn fmt_cents(v: i64) -> String {
    let sign = if v < 0 { "-" } else { "" };
    let abs = v.abs();
    format!("{sign}{}.{:02}", abs / 100, abs % 100)
}

fn fmt_bp_pct(bp: i32) -> String {
    if bp % 100 == 0 {
        format!("{}%", bp / 100)
    } else {
        format!("{:.1}%", bp as f64 / 100.0)
    }
}

fn fmt_qty_milli(q: i64) -> String {
    let sign = if q < 0 { "-" } else { "" };
    let abs = q.abs();
    let units = abs / 1000;
    let frac = abs % 1000;
    if frac == 0 {
        return format!("{sign}{units}");
    }
    let s = if frac % 100 == 0 {
        format!("{}", frac / 100)
    } else if frac % 10 == 0 {
        format!("{:02}", frac / 10)
    } else {
        format!("{:03}", frac)
    };
    format!("{sign}{units}.{s}")
}

fn pad_right(s: &str, n: usize) -> String {
    let mut out: String = s.chars().take(n).collect();
    while out.chars().count() < n {
        out.push(' ');
    }
    out
}

fn pad_left(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count >= n {
        return s.chars().take(n).collect();
    }
    let mut out = " ".repeat(n - count);
    out.push_str(s);
    out
}

fn center(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count >= n {
        return s.chars().take(n).collect();
    }
    let total = n - count;
    let left = total / 2;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(total - left))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Align {
    Left,
    Center,
    Right,
}

/// Modo de renderização: texto puro (impressora genérica/ecrã/ficheiro) ou com
/// marcadores de estilo embebidos (consumidos pelo encoder ESC/POS).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Plain,
    Markers,
}

/// Marcadores de estilo de largura-zero (área de uso privado Unicode) emitidos
/// no modo [`RenderMode::Markers`]. O encoder ESC/POS traduz-os em sequências;
/// o serializador de texto puro ignora-os.
pub mod markers {
    pub const RED_ON: char = '\u{E000}';
    pub const RED_OFF: char = '\u{E001}';
    pub const DOUBLE_ON: char = '\u{E002}';
    pub const DOUBLE_OFF: char = '\u{E003}';
    pub const UNDER_ON: char = '\u{E004}';
    pub const UNDER_OFF: char = '\u{E005}';

    pub fn is_marker(c: char) -> bool {
        ('\u{E000}'..='\u{E005}').contains(&c)
    }
}

/// Comprimento visível (ignora marcadores de estilo de largura-zero).
fn visible_len(s: &str) -> usize {
    s.chars().filter(|c| !markers::is_marker(*c)).count()
}

fn apply_align(s: &str, align: Align, width: usize) -> String {
    match align {
        Align::Left => s.to_string(),
        Align::Center => {
            let t = s.trim();
            let v = visible_len(t);
            if v >= width {
                return t.to_string();
            }
            let total = width - v;
            let left = total / 2;
            format!("{}{}{}", " ".repeat(left), t, " ".repeat(total - left))
        }
        Align::Right => {
            let t = s.trim();
            let v = visible_len(t);
            if v >= width {
                return t.to_string();
            }
            format!("{}{}", " ".repeat(width - v), t)
        }
    }
}

// ---------------------------------------------------------------------------
// Catálogo de flags conhecidas
// ---------------------------------------------------------------------------

/// Flags reconhecidas pelo tokenizer (sem a barra). A correspondência é feita
/// pelo prefixo mais longo, pelo que basta a lista — não a ordem.
const KNOWN_FLAGS: &[&str] = &[
    // Casa
    "no", "ds", "mo", "lo", "cp", "pa", "tf", "fx", "cv", "nr", "cs", "nc",
    // Cliente
    "ol", "nx", "nl", "cl", "cx", "mc", "ll", "xp", "xz",
    // Data/hora
    "dt", "da", "sd", "ho", "hc", "xt",
    // Documento
    "nd", "ns", "atcud", "qr", "hash", "versao",
    // Empregado / mesa
    "ne", "oe", "nm", "om",
    // Pessoas / valor
    "np", "pp", "st", "vt", "ve", "sx", "tx", "ti",
    // Pagamento
    "vc", "vg", "fp", "tr", "te", "pg", "pe",
    // Atributos
    "a1", "a2", "a3",
    // Formatação
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9",
    // Códigos de barras / bitmaps
    "bc", "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9",
    // Outras
    "lc",
];

const MAX_FLAG_LEN: usize = 6; // "versao"

/// Dado o texto a seguir a uma `\`, devolve `(flag, consumed_chars)` da flag
/// conhecida mais longa, ou `None` se nenhuma corresponder.
fn match_flag(rest: &[char]) -> Option<(String, usize)> {
    let max = rest.len().min(MAX_FLAG_LEN);
    let mut len = max;
    while len >= 2 {
        let candidate: String = rest[..len].iter().collect();
        if KNOWN_FLAGS.contains(&candidate.as_str()) {
            return Some((candidate, len));
        }
        len -= 1;
    }
    None
}

// ---------------------------------------------------------------------------
// Resolução
// ---------------------------------------------------------------------------

fn datetime(ctx: &DocumentContext, when: Option<DateTime<Utc>>, fmt: &str) -> Option<String> {
    when.or(ctx.issued_at)
        .map(|d| d.format(fmt).to_string())
}

/// Resolve uma flag para o seu valor textual. As flags de bloco (`\ti`, `\qr`)
/// e de formatação (`\s*`) são tratadas pelo caller e não passam por aqui.
fn resolve_flag(name: &str, ctx: &DocumentContext, line: Option<&LineContext>) -> Option<String> {
    let c = &ctx.company;
    let cli = ctx.client.as_ref();
    let v = match name {
        // Casa
        "no" => c.trade_name.clone().unwrap_or_else(|| c.legal_name.clone()),
        "ds" => c.legal_name.clone(),
        "mo" => c.address.clone(),
        "lo" => c.city.clone().unwrap_or_default(),
        "cp" => c.postal_code.clone().unwrap_or_default(),
        "pa" => c.country.clone().unwrap_or_default(),
        "tf" => c.phone.clone().unwrap_or_default(),
        "fx" => c.fax.clone().unwrap_or_default(),
        "cv" => c.registry_office.clone().unwrap_or_default(),
        "nr" => c.registry_number.clone().unwrap_or_default(),
        "cs" => c.share_capital_cents.map(fmt_cents).unwrap_or_default(),
        "nc" => c.nif.clone(),
        // Cliente
        "ol" => cli.and_then(|x| x.name.clone()).unwrap_or_default(),
        "nl" => cli.and_then(|x| x.number.clone()).unwrap_or_default(),
        "cl" => cli.and_then(|x| x.nif.clone()).unwrap_or_default(),
        "mc" => cli.and_then(|x| x.address.clone()).unwrap_or_default(),
        "ll" => cli.and_then(|x| x.city.clone()).unwrap_or_default(),
        "xp" => cli.and_then(|x| x.postal_code.clone()).unwrap_or_default(),
        "xz" => cli.and_then(|x| x.zone.clone()).unwrap_or_default(),
        "cx" => cli
            .and_then(|x| x.association_nif.clone().or_else(|| x.nif.clone()))
            .unwrap_or_default(),
        // Documento. NOTA: a spec lista `\nx` em duas secções (cliente-ou-
        // associação E Série/Número). Resolvemos como identificador composto
        // do documento, que é o uso útil em cabeçalhos fiscais.
        "nx" => match (&ctx.series, &ctx.document_number) {
            (Some(s), Some(n)) => format!("{s}/{n}"),
            (Some(s), None) => s.clone(),
            (None, Some(n)) => n.clone(),
            (None, None) => String::new(),
        },
        "nd" => ctx.document_number.clone().unwrap_or_default(),
        "ns" => ctx.series.clone().unwrap_or_default(),
        "atcud" => ctx.atcud.clone().unwrap_or_default(),
        "hash" => ctx.hash_short.clone().unwrap_or_default(),
        "versao" => ctx.software_version.clone().unwrap_or_default(),
        // Data/hora
        "dt" => datetime(ctx, ctx.issued_at, "%Y-%m-%d")?,
        "da" => datetime(ctx, ctx.opened_at, "%Y-%m-%d")?,
        "sd" => datetime(ctx, ctx.now, "%Y-%m-%d")?,
        "ho" => datetime(ctx, ctx.issued_at, "%H:%M:%S")?,
        "hc" => datetime(ctx, ctx.issued_at, "%H:%M")?,
        "xt" => ctx.issued_at.map(|d| d.to_rfc3339())?,
        // Empregado / mesa
        "ne" => ctx.employee.number.clone().unwrap_or_default(),
        "oe" => ctx.employee.name.clone().unwrap_or_default(),
        "nm" => ctx.table_number.clone().unwrap_or_default(),
        "om" => ctx.table_name.clone().unwrap_or_default(),
        // Pessoas / valor
        "np" => ctx.num_people.map(|n| n.to_string()).unwrap_or_default(),
        "pp" => match ctx.num_people {
            Some(n) if n > 0 => fmt_cents(ctx.total / n),
            _ => String::new(),
        },
        "st" => fmt_cents(ctx.subtotal),
        "vt" => fmt_cents(ctx.total),
        "ve" => ctx
            .secondary_rate
            .map(|r| fmt_cents((ctx.total as f64 * r).round() as i64))
            .unwrap_or_default(),
        "sx" => fmt_cents(ctx.total_sem_iva),
        "tx" => fmt_cents(ctx.iva_total),
        // Pagamento
        "vg" => fmt_cents(ctx.gorjeta),
        "fp" => ctx.payments.first().map(|p| p.method.clone()).unwrap_or_default(),
        "tr" => fmt_cents(ctx.troco),
        "te" => ctx
            .secondary_rate
            .map(|r| fmt_cents((ctx.troco as f64 * r).round() as i64))
            .unwrap_or_default(),
        "pg" => fmt_cents(ctx.pago),
        "pe" => ctx
            .secondary_rate
            .map(|r| fmt_cents((ctx.pago as f64 * r).round() as i64))
            .unwrap_or_default(),
        "vc" => String::new(), // requer factor de conversão por método (n/d)
        // Atributos
        "a1" => ctx.a1.clone().unwrap_or_default(),
        "a2" => ctx.a2.clone().unwrap_or_default(),
        "a3" => ctx.a3.clone().unwrap_or_default(),
        // Códigos de barras: sem efeito em texto simples.
        "bc" | "b0" | "b1" | "b2" | "b3" | "b4" | "b5" | "b6" | "b7" | "b8" | "b9" => String::new(),
        // Outras
        "lc" => ctx.local_name.clone().unwrap_or_default(),
        // Campos de linha quando aplicável (resolvidos em resolve_field também)
        _ => return resolve_line_flag(name, line),
    };
    Some(v)
}

fn resolve_line_flag(_name: &str, _line: Option<&LineContext>) -> Option<String> {
    None
}

/// Resolve `<! type="field" id="…" !>`. Suporta os caminhos do catálogo
/// (`fb_c_*`, `fb_d_*`, `memp_*`, `mcli_*`) e alguns caminhos com `.`.
fn resolve_field(path: &str, ctx: &DocumentContext, line: Option<&LineContext>) -> Option<String> {
    // Caminhos com descida por relação: resolvemos pelo sufixo conhecido.
    if let Some((_, sub)) = path.split_once('.') {
        return resolve_field(sub, ctx, line);
    }
    let cli = ctx.client.as_ref();
    let v = match path {
        // Cabeçalho do documento
        "fb_c_proc" => ctx.document_number.clone().unwrap_or_default(),
        "fb_c_data" => datetime(ctx, ctx.issued_at, "%Y-%m-%d")?,
        "fb_c_hora" => datetime(ctx, ctx.issued_at, "%H:%M:%S")?,
        "fb_c_mesa" => ctx.table_number.clone().unwrap_or_default(),
        "fb_c_nome_mesa" => ctx.table_name.clone().unwrap_or_default(),
        "fb_c_nif" => cli.and_then(|x| x.nif.clone()).unwrap_or_default(),
        "fb_c_np" => ctx.num_people.map(|n| n.to_string()).unwrap_or_default(),
        "fb_c_vtot" => fmt_cents(ctx.total),
        "fb_c_vsiva" => fmt_cents(ctx.total_sem_iva),
        "fb_c_a1" => ctx.a1.clone().unwrap_or_default(),
        "fb_c_a2" => ctx.a2.clone().unwrap_or_default(),
        "fb_c_a3" => ctx.a3.clone().unwrap_or_default(),
        // Detalhe (necessita da linha corrente)
        "fb_d_qtd" => fmt_qty_milli(line?.qty_milli),
        "fb_d_art" => line?.article_code.clone().unwrap_or_default(),
        "fb_d_design" => line?.name.clone(),
        "fb_d_nome_curto" => line?.short_name.clone().unwrap_or_else(|| line.map(|l| l.name.clone()).unwrap_or_default()),
        "fb_d_punit" => fmt_cents(line?.unit_price),
        "fb_d_pcusto" => fmt_cents(line?.price_sem_iva),
        "fb_d_perc_desc" => fmt_bp_pct(line?.perc_desc_bp),
        "fb_d_val_desc" => fmt_cents(line?.val_desc),
        "fb_d_iva_cod" => line?.iva_cod.clone().unwrap_or_default(),
        "fb_d_iva_perc" => fmt_bp_pct(line?.iva_perc_bp),
        "fb_d_total_linha" => fmt_cents(line?.total),
        "fb_d_zona_imp" => line?.zona_imp.clone().unwrap_or_default(),
        // Empregado / cliente
        "memp_nome" => ctx.employee.name.clone().unwrap_or_default(),
        "memp_codigo" => ctx.employee.number.clone().unwrap_or_default(),
        "mcli_nome" => cli.and_then(|x| x.name.clone()).unwrap_or_default(),
        "mcli_nif" => cli.and_then(|x| x.nif.clone()).unwrap_or_default(),
        "mcli_morada" => cli.and_then(|x| x.address.clone()).unwrap_or_default(),
        _ => return None,
    };
    Some(v)
}

// ---------------------------------------------------------------------------
// Construção XML-like
// ---------------------------------------------------------------------------

struct XmlConstruct {
    type_: String,
    id: String,
    mask: Option<String>,
    align: Option<String>,
    default: Option<String>,
    offset: Option<i64>,
}

/// Lê um atributo `nome="valor"` simples.
fn parse_attr(s: &str, name: &str) -> Option<String> {
    let key = format!("{name}=\"");
    let start = s.find(&key)? + key.len();
    let end = s[start..].find('"')? + start;
    Some(s[start..end].to_string())
}

fn parse_xml(body: &str) -> Option<XmlConstruct> {
    Some(XmlConstruct {
        type_: parse_attr(body, "type")?,
        id: parse_attr(body, "id").unwrap_or_default(),
        mask: parse_attr(body, "mask"),
        align: parse_attr(body, "align"),
        default: parse_attr(body, "default"),
        offset: parse_attr(body, "offset").and_then(|o| o.parse().ok()),
    })
}

fn render_xml(x: &XmlConstruct, ctx: &DocumentContext, line: Option<&LineContext>) -> String {
    let raw = match x.type_.as_str() {
        "flag" => resolve_flag(&x.id, ctx, line),
        "field" => resolve_field(&x.id, ctx, line),
        "uid" => Some(gen_uid(&x.id, x.offset.unwrap_or(0))),
        _ => None,
    };
    let mut value = raw.filter(|s| !s.is_empty()).or_else(|| x.default.clone()).unwrap_or_default();

    if let Some(mask) = &x.mask {
        let width = mask.chars().count();
        let align = match x.align.as_deref() {
            Some("right") => Align::Right,
            Some("center") => Align::Center,
            _ => Align::Left,
        };
        value = match align {
            Align::Left => pad_right(&value, width),
            Align::Right => pad_left(&value, width),
            Align::Center => center(&value, width),
        };
    }
    value
}

/// UID determinístico para senhas de refeição: `id` numérico somado ao offset.
fn gen_uid(id: &str, offset: i64) -> String {
    match id.parse::<i64>() {
        Ok(n) => format!("{}", n + offset),
        Err(_) => id.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

/// Renderiza uma única linha do template (sem blocos). Devolve a linha já
/// alinhada à largura. No modo [`RenderMode::Markers`], os estilos `\s0`..`\s5`
/// emitem marcadores de largura-zero para o encoder ESC/POS.
fn render_line_mode(
    line: &str,
    ctx: &DocumentContext,
    lctx: Option<&LineContext>,
    width: usize,
    mode: RenderMode,
) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::new();
    let mut align = Align::Left;
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '<' && chars.get(i + 1) == Some(&'!') {
            // Procura o fecho "!>".
            if let Some(end) = find_seq(&chars, i + 2, &['!', '>']) {
                let body: String = chars[i + 2..end].iter().collect();
                if let Some(x) = parse_xml(&body) {
                    out.push_str(&render_xml(&x, ctx, lctx));
                }
                i = end + 2;
                continue;
            }
        }
        if chars[i] == '\\' {
            if let Some((flag, consumed)) = match_flag(&chars[i + 1..]) {
                i += 1 + consumed;
                match flag.as_str() {
                    "s7" => align = Align::Center,
                    "s8" => align = Align::Right,
                    "s9" => align = Align::Left,
                    "s0" | "s1" | "s2" | "s3" | "s4" | "s5" => {
                        if mode == RenderMode::Markers {
                            out.push(match flag.as_str() {
                                "s0" => markers::RED_ON,
                                "s1" => markers::RED_OFF,
                                "s2" => markers::DOUBLE_ON,
                                "s3" => markers::DOUBLE_OFF,
                                "s4" => markers::UNDER_ON,
                                _ => markers::UNDER_OFF,
                            });
                        }
                    }
                    // \s6 (início de código de barras EAN-13): sem efeito aqui.
                    "s6" => {}
                    _ => {
                        if let Some(val) = resolve_flag(&flag, ctx, lctx) {
                            out.push_str(&val);
                        }
                    }
                }
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    apply_align(&out, align, width)
}

fn find_seq(chars: &[char], from: usize, seq: &[char]) -> Option<usize> {
    let mut i = from;
    while i + seq.len() <= chars.len() {
        if chars[i..i + seq.len()] == *seq {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Renderiza a tabela de IVA (`\ti`).
fn render_vat_table(ctx: &DocumentContext, width: usize) -> String {
    if ctx.vat_rows.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    // Larguras: Taxa | Base | IVA | Total
    let w_taxa = 8;
    let rest = width.saturating_sub(w_taxa);
    let col = rest / 3;
    out.push_str(&pad_right("Taxa", w_taxa));
    out.push_str(&pad_left("Base", col));
    out.push_str(&pad_left("IVA", col));
    out.push_str(&pad_left("Total", rest - 2 * col));
    out.push('\n');
    for r in &ctx.vat_rows {
        out.push_str(&pad_right(&r.label, w_taxa));
        out.push_str(&pad_left(&fmt_cents(r.base), col));
        out.push_str(&pad_left(&fmt_cents(r.vat), col));
        out.push_str(&pad_left(&fmt_cents(r.base + r.vat), rest - 2 * col));
        out.push('\n');
    }
    out.pop(); // remove o último \n (o caller junta-o)
    out
}

/// Renderiza um bloco de template (cabeçalho/detalhe/rodapé). Linhas que contêm
/// apenas `\ti` ou `\qr` expandem-se em vários renglones.
fn render_block(
    template: &str,
    ctx: &DocumentContext,
    lctx: Option<&LineContext>,
    width: usize,
    mode: RenderMode,
) -> String {
    let mut out = String::new();
    for raw in template.split('\n') {
        let trimmed = raw.trim();
        if trimmed == "\\ti" {
            let t = render_vat_table(ctx, width);
            if !t.is_empty() {
                out.push_str(&t);
                out.push('\n');
            }
            continue;
        }
        if trimmed == "\\qr" {
            if !ctx.qr_block.is_empty() {
                out.push_str(&ctx.qr_block);
                if !ctx.qr_block.ends_with('\n') {
                    out.push('\n');
                }
            }
            if !ctx.qr_payload.is_empty() {
                out.push_str(&center("[QR Code]", width));
                out.push('\n');
                out.push_str(&ctx.qr_payload);
                out.push('\n');
            }
            continue;
        }
        out.push_str(&render_line_mode(raw, ctx, lctx, width, mode));
        out.push('\n');
    }
    out
}

/// Renderiza um documento completo em texto puro.
pub fn render_document(tpl: &DocumentTemplate, ctx: &DocumentContext, width: usize) -> String {
    render_document_mode(tpl, ctx, width, RenderMode::Plain)
}

/// Renderiza um documento completo: cabeçalho, linhas de detalhe (uma por cada
/// `LineContext`), e rodapé. O `mode` controla se os estilos são descartados
/// (texto puro) ou emitidos como marcadores para o encoder ESC/POS.
pub fn render_document_mode(
    tpl: &DocumentTemplate,
    ctx: &DocumentContext,
    width: usize,
    mode: RenderMode,
) -> String {
    let mut out = String::new();
    if !tpl.cabecalho.is_empty() {
        out.push_str(&render_block(&tpl.cabecalho, ctx, None, width, mode));
    }
    if !tpl.nao_imprime_detalhes && !tpl.linha_detalhe.is_empty() {
        for line in &ctx.lines {
            out.push_str(&render_block(&tpl.linha_detalhe, ctx, Some(line), width, mode));
        }
    }
    if !tpl.rodape.is_empty() {
        out.push_str(&render_block(&tpl.rodape, ctx, None, width, mode));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Wrapper de conveniência para os testes: renderiza uma linha em texto puro.
    fn render_line(line: &str, ctx: &DocumentContext, lctx: Option<&LineContext>, width: usize) -> String {
        render_line_mode(line, ctx, lctx, width, RenderMode::Plain)
    }

    fn sample_ctx() -> DocumentContext {
        DocumentContext {
            company: Company {
                legal_name: "Tasca do Zé, Lda".into(),
                trade_name: Some("Tasca do Zé".into()),
                nif: "501234567".into(),
                address: "Rua Direita, 10".into(),
                city: Some("Porto".into()),
                postal_code: Some("4000-001".into()),
                country: Some("PT".into()),
                share_capital_cents: Some(500000),
                ..Default::default()
            },
            issued_at: Some(Utc.with_ymd_and_hms(2026, 5, 29, 14, 30, 15).unwrap()),
            document_number: Some("12".into()),
            series: Some("FS2026".into()),
            atcud: Some("ABCD1234-12".into()),
            hash_short: Some("AB12".into()),
            num_people: Some(2),
            subtotal: 1000,
            total: 1130,
            total_sem_iva: 1000,
            iva_total: 130,
            table_number: Some("5".into()),
            table_name: Some("Mesa 5".into()),
            employee: Staff { number: Some("1".into()), name: Some("Ana".into()) },
            payments: vec![PaymentLine { method: "Numerário".into(), amount: 1130 }],
            lines: vec![
                LineContext {
                    qty_milli: 2000,
                    name: "Café".into(),
                    unit_price: 80,
                    total: 160,
                    iva_perc_bp: 1300,
                    ..Default::default()
                },
                LineContext {
                    qty_milli: 1500,
                    name: "Bola de Berlim".into(),
                    unit_price: 120,
                    total: 180,
                    iva_perc_bp: 2300,
                    ..Default::default()
                },
            ],
            vat_rows: vec![VatRow { label: "13%".into(), base: 1000, vat: 130 }],
            ..Default::default()
        }
    }

    #[test]
    fn flag_company_and_doc() {
        let ctx = sample_ctx();
        assert_eq!(render_line("\\no", &ctx, None, 48).trim(), "Tasca do Zé");
        assert_eq!(render_line("\\ds", &ctx, None, 48).trim(), "Tasca do Zé, Lda");
        assert_eq!(render_line("NIF: \\nc", &ctx, None, 48).trim(), "NIF: 501234567");
        assert_eq!(render_line("\\nx", &ctx, None, 48).trim(), "FS2026/12");
        assert_eq!(render_line("\\nd", &ctx, None, 48).trim(), "12");
        assert_eq!(render_line("Cap: \\cs", &ctx, None, 48).trim(), "Cap: 5000.00");
    }

    #[test]
    fn flag_datetime() {
        let ctx = sample_ctx();
        assert_eq!(render_line("\\dt", &ctx, None, 48).trim(), "2026-05-29");
        assert_eq!(render_line("\\ho", &ctx, None, 48).trim(), "14:30:15");
        assert_eq!(render_line("\\hc", &ctx, None, 48).trim(), "14:30");
    }

    #[test]
    fn flag_values() {
        let ctx = sample_ctx();
        assert_eq!(render_line("Total \\vt", &ctx, None, 48).trim(), "Total 11.30");
        assert_eq!(render_line("Sem IVA \\sx", &ctx, None, 48).trim(), "Sem IVA 10.00");
        assert_eq!(render_line("IVA \\tx", &ctx, None, 48).trim(), "IVA 1.30");
        assert_eq!(render_line("\\np pessoas", &ctx, None, 48).trim(), "2 pessoas");
        // por pessoa = 1130/2 = 565
        assert_eq!(render_line("\\pp", &ctx, None, 48).trim(), "5.65");
    }

    #[test]
    fn alignment_flags() {
        let ctx = sample_ctx();
        let centered = render_line("\\s7TITULO", &ctx, None, 10);
        assert_eq!(centered, "  TITULO  ");
        let right = render_line("\\s8X", &ctx, None, 5);
        assert_eq!(right, "    X");
        let left = render_line("\\s9X", &ctx, None, 5);
        assert_eq!(left, "X");
    }

    #[test]
    fn style_flags_stripped() {
        let ctx = sample_ctx();
        // \s2/\s3 (tamanho duplo) não deixam resíduo no texto simples.
        assert_eq!(render_line("\\s2Total\\s3", &ctx, None, 48).trim(), "Total");
    }

    #[test]
    fn xml_field_with_mask_align() {
        let ctx = sample_ctx();
        // Total alinhado à direita num campo de 10.
        let out = render_line(
            "<! type=\"field\" id=\"fb_c_vtot\" align=\"right\" mask=\"##########\" !>",
            &ctx,
            None,
            48,
        );
        assert_eq!(out, "     11.30");
    }

    #[test]
    fn xml_default_when_empty() {
        let ctx = sample_ctx();
        let out = render_line(
            "<! type=\"flag\" id=\"tf\" default=\"sem telefone\" !>",
            &ctx,
            None,
            48,
        );
        assert_eq!(out.trim(), "sem telefone");
    }

    #[test]
    fn xml_uid_offset() {
        let ctx = sample_ctx();
        let out = render_line("<! type=\"uid\" id=\"100\" offset=\"5000\" !>", &ctx, None, 48);
        assert_eq!(out.trim(), "5100");
    }

    #[test]
    fn detail_line_fields() {
        let ctx = sample_ctx();
        let tpl = DocumentTemplate {
            cabecalho: String::new(),
            linha_detalhe: "<! type=\"field\" id=\"fb_d_qtd\" mask=\"###\" align=\"right\" !> <! type=\"field\" id=\"fb_d_design\" mask=\"####################\" !><! type=\"field\" id=\"fb_d_total_linha\" mask=\"########\" align=\"right\" !>".into(),
            rodape: String::new(),
            nao_imprime_detalhes: false,
        };
        let out = render_document(&tpl, &ctx, 48);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("  2 Café"));
        assert!(lines[0].trim_end().ends_with("1.60"));
        assert!(lines[1].starts_with("1.5 Bola de Berlim"));
        assert!(lines[1].trim_end().ends_with("1.80"));
    }

    #[test]
    fn vat_table_block() {
        let ctx = sample_ctx();
        let tpl = DocumentTemplate {
            cabecalho: String::new(),
            linha_detalhe: String::new(),
            rodape: "\\ti".into(),
            nao_imprime_detalhes: true,
        };
        let out = render_document(&tpl, &ctx, 48);
        assert!(out.contains("Taxa"));
        assert!(out.contains("13%"));
        assert!(out.contains("10.00"));
        assert!(out.contains("1.30"));
    }

    #[test]
    fn full_document_skips_detail_when_flagged() {
        let ctx = sample_ctx();
        let tpl = DocumentTemplate {
            cabecalho: "\\s7\\no".into(),
            linha_detalhe: "\\nm".into(),
            rodape: "TOTAL \\vt".into(),
            nao_imprime_detalhes: true,
        };
        let out = render_document(&tpl, &ctx, 48);
        assert!(out.contains("Tasca do Zé"));
        assert!(out.contains("TOTAL 11.30"));
        // detalhe saltado: não repete "5" duas vezes do \nm
        assert!(!out.contains("\n5\n"));
    }

    #[test]
    fn unknown_flag_preserved() {
        let ctx = sample_ctx();
        // \zz não é conhecida: mantém-se literal.
        assert_eq!(render_line("a\\zzb", &ctx, None, 48).trim(), "a\\zzb");
    }
}
