use chrono::{DateTime, Utc};

pub struct ReceiptLine<'a> {
    pub name: &'a str,
    pub qty: i32,
    pub unit_price: i64,
    pub total: i64,
    pub vat_label: String,
}

pub struct VatRow {
    pub label: String,
    pub base: i64,
    pub vat: i64,
}

pub struct ReceiptCtx<'a> {
    pub company_legal_name: &'a str,
    pub company_trade_name: Option<&'a str>,
    pub company_nif: &'a str,
    pub company_address: &'a str,
    pub company_postal_city: &'a str,
    pub company_share_capital_cents: Option<i64>,
    pub company_registry: Option<(&'a str, &'a str)>,
    pub terminal: &'a str,
    pub table_label: &'a str,
    pub document_type_label: &'a str,
    pub document_identifier: &'a str,
    pub atcud: &'a str,
    pub hash_short: &'a str,
    pub software_certificate: &'a str,
    pub issued_at: DateTime<Utc>,
    pub lines: Vec<ReceiptLine<'a>>,
    pub vat_rows: Vec<VatRow>,
    pub total: i64,
    pub payments: Vec<(String, i64)>,
    pub troco_cents: i64,
    pub qr_block: &'a str,
    pub qr_payload: &'a str,
}

const WIDTH: usize = 48;

fn fmt_cents(v: i64) -> String {
    let sign = if v < 0 { "-" } else { "" };
    let abs = v.abs();
    format!("{sign}{}.{:02}", abs / 100, abs % 100)
}

fn pad_right(s: &str, n: usize) -> String {
    let mut out = s.chars().take(n).collect::<String>();
    while out.chars().count() < n {
        out.push(' ');
    }
    out
}

fn pad_left(s: &str, n: usize) -> String {
    let count = s.chars().count();
    let mut out = String::new();
    if count < n {
        for _ in 0..(n - count) {
            out.push(' ');
        }
    }
    out.push_str(s);
    out
}

fn center(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count >= n {
        return s.to_string();
    }
    let total = n - count;
    let left = total / 2;
    let right = total - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

fn sep() -> String {
    "-".repeat(WIDTH)
}

pub struct KitchenLine<'a> {
    pub qty: i32,
    pub name: &'a str,
}

/// Bloco de pedido cruzado: "sai junto com" os itens desta outra zona.
pub struct CrossZoneBlock<'a> {
    pub zona: &'a str,
    pub lines: &'a [KitchenLine<'a>],
}

/// Formata um talão de pedido para uma zona (cozinha/bar). Sem totais nem IVA.
/// `cross_zones` lista itens que saem em paralelo em outras zonas marcadas como
/// secundárias (spec 10.3 `secundarios=true`) — aparecem em letra "pequena"
/// (indentação + prefixo `*`).
pub fn format_kitchen_ticket(
    zona: &str,
    local: &str,
    table_label: &str,
    when: DateTime<Utc>,
    lines: &[KitchenLine<'_>],
    cross_zones: &[CrossZoneBlock<'_>],
) -> String {
    let mut out = String::new();
    out.push_str(&center(&format!("== {} ==", zona.to_uppercase()), WIDTH));
    out.push('\n');
    out.push_str(&sep());
    out.push('\n');
    out.push_str(&pad_right(&format!("{}  {}", local, table_label), WIDTH));
    out.push('\n');
    out.push_str(&pad_right(&when.format("%Y-%m-%d %H:%M:%S").to_string(), WIDTH));
    out.push('\n');
    out.push_str(&sep());
    out.push('\n');
    for line in lines {
        let qty = format!("{}x", line.qty);
        out.push_str(&pad_left(&qty, 3));
        out.push(' ');
        out.push_str(&pad_right(line.name, WIDTH - 4));
        out.push('\n');
    }
    out.push_str(&sep());
    out.push('\n');

    if !cross_zones.is_empty() {
        out.push_str(&center("-- Sai junto com --", WIDTH));
        out.push('\n');
        for block in cross_zones {
            out.push_str(&pad_right(&format!("  [{}]", block.zona), WIDTH));
            out.push('\n');
            for line in block.lines {
                let qty = format!("{}x", line.qty);
                out.push_str("    ");
                out.push_str(&pad_left(&qty, 3));
                out.push(' ');
                out.push_str(&pad_right(line.name, WIDTH - 8));
                out.push('\n');
            }
        }
        out.push_str(&sep());
        out.push('\n');
    }

    out
}

/// Ticket de anulação para a zona original (cozinha/bar).
pub fn format_anulacao_ticket(
    zona: &str,
    local: &str,
    table_label: &str,
    when: DateTime<Utc>,
    qty: i32,
    artigo: &str,
    com_desperdicio: bool,
    motivo: Option<&str>,
) -> String {
    let mut out = String::new();
    out.push_str(&center(&format!("** ANULAÇÃO {} **", zona.to_uppercase()), WIDTH));
    out.push('\n');
    out.push_str(&sep());
    out.push('\n');
    out.push_str(&pad_right(&format!("{}  {}", local, table_label), WIDTH));
    out.push('\n');
    out.push_str(&pad_right(&when.format("%Y-%m-%d %H:%M:%S").to_string(), WIDTH));
    out.push('\n');
    out.push_str(&sep());
    out.push('\n');
    let qty_s = format!("{}x", qty);
    out.push_str(&pad_left(&qty_s, 3));
    out.push(' ');
    out.push_str(&pad_right(artigo, WIDTH - 4));
    out.push('\n');
    out.push_str(&center(
        if com_desperdicio { "(com desperdício)" } else { "(sem desperdício)" },
        WIDTH,
    ));
    out.push('\n');
    if let Some(m) = motivo {
        if !m.is_empty() {
            out.push_str(&pad_right(&format!("Motivo: {}", m), WIDTH));
            out.push('\n');
        }
    }
    out.push_str(&sep());
    out.push('\n');
    out
}

pub fn format_legal_receipt(ctx: ReceiptCtx<'_>) -> String {
    let mut out = String::new();

    // Cabeçalho da empresa
    if let Some(trade) = ctx.company_trade_name {
        out.push_str(&center(trade, WIDTH));
        out.push('\n');
    }
    out.push_str(&center(ctx.company_legal_name, WIDTH));
    out.push('\n');
    out.push_str(&center(ctx.company_address, WIDTH));
    out.push('\n');
    if !ctx.company_postal_city.is_empty() {
        out.push_str(&center(ctx.company_postal_city, WIDTH));
        out.push('\n');
    }
    out.push_str(&center(&format!("NIF: {}", ctx.company_nif), WIDTH));
    out.push('\n');
    if let Some((office, number)) = ctx.company_registry {
        out.push_str(&center(&format!("{}: {}", office, number), WIDTH));
        out.push('\n');
    }
    if let Some(capital) = ctx.company_share_capital_cents {
        out.push_str(&center(
            &format!("Capital Social: {} EUR", fmt_cents(capital)),
            WIDTH,
        ));
        out.push('\n');
    }

    out.push_str(&sep());
    out.push('\n');
    out.push_str(&center(ctx.document_type_label, WIDTH));
    out.push('\n');
    out.push_str(&center(ctx.document_identifier, WIDTH));
    out.push('\n');
    out.push_str(&sep());
    out.push('\n');

    out.push_str(&pad_right(
        &format!("{}  {}", ctx.table_label, ctx.terminal),
        WIDTH,
    ));
    out.push('\n');
    out.push_str(&pad_right(
        &ctx.issued_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        WIDTH,
    ));
    out.push('\n');
    out.push_str(&pad_right("Adquirente: Consumidor Final", WIDTH));
    out.push('\n');
    out.push_str(&sep());
    out.push('\n');

    // Linhas
    let name_col = WIDTH - 16;
    out.push_str(&pad_right("Qtd Artigo", name_col));
    out.push_str(&pad_left("IVA", 5));
    out.push_str(&pad_left("Total", 11));
    out.push('\n');
    for line in &ctx.lines {
        let qty = format!("{}x", line.qty);
        let row = format!(
            "{} {}",
            pad_left(&qty, 3),
            pad_right(line.name, name_col - 4),
        );
        out.push_str(&pad_right(&row, name_col));
        out.push_str(&pad_left(&line.vat_label, 5));
        out.push_str(&pad_left(&fmt_cents(line.total), 11));
        out.push('\n');
    }

    out.push_str(&sep());
    out.push('\n');
    out.push_str(&pad_right("TOTAL", WIDTH - 12));
    out.push_str(&pad_left(&format!("{} EUR", fmt_cents(ctx.total)), 12));
    out.push('\n');

    // IVA breakdown
    if !ctx.vat_rows.is_empty() {
        out.push_str(&sep());
        out.push('\n');
        out.push_str(&pad_right("Taxa", 8));
        out.push_str(&pad_left("Base", 14));
        out.push_str(&pad_left("IVA", 12));
        out.push_str(&pad_left("Total", 14));
        out.push('\n');
        for row in &ctx.vat_rows {
            out.push_str(&pad_right(&row.label, 8));
            out.push_str(&pad_left(&fmt_cents(row.base), 14));
            out.push_str(&pad_left(&fmt_cents(row.vat), 12));
            out.push_str(&pad_left(&fmt_cents(row.base + row.vat), 14));
            out.push('\n');
        }
    }

    if !ctx.payments.is_empty() {
        out.push_str(&sep());
        out.push('\n');
        for (method, amount) in &ctx.payments {
            out.push_str(&pad_right(method, WIDTH - 12));
            out.push_str(&pad_left(&fmt_cents(*amount), 12));
            out.push('\n');
        }
        if ctx.troco_cents > 0 {
            out.push_str(&pad_right("Troco", WIDTH - 12));
            out.push_str(&pad_left(&fmt_cents(ctx.troco_cents), 12));
            out.push('\n');
        }
    }

    out.push_str(&sep());
    out.push('\n');
    out.push_str(&pad_right("ATCUD:", 8));
    out.push_str(ctx.atcud);
    out.push('\n');
    out.push_str(&pad_right("Hash:", 8));
    out.push_str(ctx.hash_short);
    out.push('\n');
    out.push_str(&center(
        &format!("Processado pelo software certificado n.º {}", ctx.software_certificate),
        WIDTH,
    ));
    out.push('\n');
    out.push_str(&sep());
    out.push('\n');

    // QR — bloco gráfico se conseguimos renderizar, sempre o payload textual.
    if !ctx.qr_block.is_empty() {
        out.push_str(ctx.qr_block);
    }
    if !ctx.qr_payload.is_empty() {
        out.push_str(&center("[QR Code payload]", WIDTH));
        out.push('\n');
        out.push_str(ctx.qr_payload);
        out.push('\n');
    }

    out
}
