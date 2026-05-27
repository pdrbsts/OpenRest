use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use server::{AppState, CompanyConfig, SystemEvent};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use domain::{
    Article, Atcud, Document, DocumentDetail, DocumentSeries, Employee, Family, Payment,
    PaymentMethod, Table,
};

mod error;
use error::ApiError;

pub type ApiResult<T> = Result<T, ApiError>;

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health))
        .route("/api/catalog", get(get_catalog))
        .route("/api/tables", get(get_tables))
        .route("/api/tables/:id/open", post(open_table))
        .route("/api/tables/:id/document", get(get_table_document))
        .route("/api/employees", get(get_employees))
        .route("/api/payment-methods", get(get_payment_methods))
        .route("/api/series", get(get_series))
        .route("/api/atcuds", get(get_atcuds))
        .route("/api/documents/:id", get(get_document))
        .route("/api/documents/:id/lines", post(add_line))
        .route("/api/documents/:id/close", post(close_document))
        .route("/api/documents/:id/print", post(print_document))
        .with_state(state)
        .layer(cors)
}

async fn health() -> &'static str {
    "ok"
}

#[derive(Serialize)]
pub struct CatalogResponse {
    pub families: Vec<Family>,
    pub articles: Vec<Article>,
}

#[derive(Serialize)]
pub struct DocumentResponse {
    pub document: Document,
    pub lines: Vec<DocumentDetail>,
    pub payments: Vec<Payment>,
}

async fn build_doc_response(
    pool: &storage::SqlitePool,
    document: Document,
) -> Result<DocumentResponse, ApiError> {
    let lines = storage::list_document_details(pool, document.id).await?;
    let payments = storage::list_document_payments(pool, document.id).await?;
    Ok(DocumentResponse { document, lines, payments })
}

async fn get_catalog(State(state): State<Arc<AppState>>) -> ApiResult<Json<CatalogResponse>> {
    let pool = state.db.pool();
    Ok(Json(CatalogResponse {
        families: storage::list_families(pool).await?,
        articles: storage::list_articles(pool).await?,
    }))
}

async fn get_tables(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Table>>> {
    Ok(Json(storage::list_tables(state.db.pool()).await?))
}

async fn get_employees(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Employee>>> {
    Ok(Json(storage::list_employees(state.db.pool()).await?))
}

async fn get_payment_methods(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<PaymentMethod>>> {
    Ok(Json(storage::list_payment_methods(state.db.pool()).await?))
}

async fn get_series(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<DocumentSeries>>> {
    Ok(Json(storage::list_series(state.db.pool()).await?))
}

async fn get_atcuds(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Atcud>>> {
    Ok(Json(storage::list_atcuds(state.db.pool()).await?))
}

#[derive(Deserialize, Default)]
pub struct OpenTableRequest {
    pub employee_id: Option<Uuid>,
}

async fn open_table(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    body: Option<Json<OpenTableRequest>>,
) -> ApiResult<Json<DocumentResponse>> {
    let employee_id = body.and_then(|b| b.employee_id);
    let document = storage::open_table(state.db.pool(), id, employee_id).await?;
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentCreated { document_id: document.id });
    Ok(Json(build_doc_response(state.db.pool(), document).await?))
}

async fn get_table_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Option<DocumentResponse>>> {
    let pool = state.db.pool();
    let Some(document) = storage::get_open_document_for_table(pool, id).await? else {
        return Ok(Json(None));
    };
    Ok(Json(Some(build_doc_response(pool, document).await?)))
}

async fn get_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let document = storage::get_document(pool, id).await?;
    Ok(Json(build_doc_response(pool, document).await?))
}

#[derive(Deserialize)]
pub struct AddLineRequest {
    pub article_id: Uuid,
    pub qty: i32,
}

async fn add_line(
    State(state): State<Arc<AppState>>,
    Path(document_id): Path<Uuid>,
    Json(req): Json<AddLineRequest>,
) -> ApiResult<Json<DocumentResponse>> {
    if req.qty <= 0 {
        return Err(ApiError::BadRequest("qty must be > 0".into()));
    }
    let pool = state.db.pool();
    let document = storage::get_document(pool, document_id).await?;
    if document.is_closed {
        return Err(ApiError::BadRequest("document is closed".into()));
    }
    let article = storage::get_article(pool, req.article_id).await?;
    storage::add_document_line(pool, document_id, article.id, req.qty, article.price).await?;
    let document = storage::get_document(pool, document_id).await?;
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentLineAdded { document_id });
    Ok(Json(build_doc_response(pool, document).await?))
}

#[derive(Deserialize, Default)]
pub struct CloseDocumentRequest {
    pub payment_method_id: Option<Uuid>,
}

async fn close_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    body: Option<Json<CloseDocumentRequest>>,
) -> ApiResult<Json<DocumentResponse>> {
    let pool = state.db.pool();
    let payment_method_id = body.and_then(|b| b.payment_method_id);
    let document = storage::get_document(pool, id).await?;
    if document.is_closed {
        return Err(ApiError::BadRequest("document already closed".into()));
    }
    if document.total <= 0 {
        return Err(ApiError::BadRequest("document has no lines".into()));
    }
    if let Some(method_id) = payment_method_id {
        storage::record_payment(pool, document.id, method_id, document.total).await?;
    }

    // Fiscal close: allocate series number, sign, build ATCUD + QR.
    let lines = storage::list_document_details(pool, document.id).await?;
    let mut articles = Vec::with_capacity(lines.len());
    for l in &lines {
        articles.push(storage::get_article(pool, l.article_id).await?);
    }
    let breakdown = compute_vat_breakdown(&lines, &articles);
    let issued_at = Utc::now();
    let year = issued_at.year();

    let mut tx = pool.begin().await?;
    let (series, atcud_entry, number) =
        storage::allocate_series_number(&mut tx, "FS", year).await?;
    let document_identifier = format!("{} {}/{}", series.document_type, series.prefix, number);
    let atcud = fiscal::atcud(&atcud_entry.atcud, number);

    let previous_hash = storage::last_hash_for_series(&mut tx, series.id).await?.unwrap_or_default();
    let payload = fiscal::signing_payload(
        issued_at,
        issued_at,
        &document_identifier,
        document.total,
        &previous_hash,
    );
    let (hash, hash_short) = fiscal::sign(&state.config.signing_key, &payload);

    let vat_lines: Vec<(fiscal::VatRate, i64, i64)> = breakdown
        .iter()
        .map(|b| (fiscal::VatRate::from_basis_points(b.rate_bp), b.base, b.vat))
        .collect();
    let total_vat: i64 = breakdown.iter().map(|b| b.vat).sum();
    let qr_payload = fiscal::qr_payload(&fiscal::QrInputs {
        emitter_nif: &state.config.company.nif,
        customer_nif: "999999990",
        country: &state.config.company.country,
        document_type: "FS",
        document_status: "N",
        document_date: issued_at,
        document_identifier: &document_identifier,
        atcud: &atcud,
        tax_country: &state.config.company.country,
        vat_breakdown: &vat_lines,
        total_vat_cents: total_vat,
        total_with_vat_cents: document.total,
        hash_short: &hash_short,
        software_certificate: &state.config.company.software_certificate,
    });

    storage::finalize_document_fiscal(
        &mut tx,
        document.id,
        series.id,
        &series.document_type,
        number,
        &atcud,
        &hash,
        &hash_short,
        &previous_hash,
        issued_at,
        &qr_payload,
    )
    .await?;

    tx.commit().await?;

    let document = storage::get_document(pool, document.id).await?;
    let _ = state
        .event_bus
        .publish(SystemEvent::DocumentClosed { document_id: document.id });
    Ok(Json(build_doc_response(pool, document).await?))
}

struct VatBucket {
    rate_bp: i32,
    base: i64,
    vat: i64,
}

fn compute_vat_breakdown(
    lines: &[DocumentDetail],
    articles: &[Article],
) -> Vec<VatBucket> {
    use std::collections::BTreeMap;
    let mut buckets: BTreeMap<i32, (i64, i64)> = BTreeMap::new();
    for (l, a) in lines.iter().zip(articles.iter()) {
        let entry = buckets.entry(a.vat_rate).or_insert((0, 0));
        // Prices stored include VAT (gross). Base = gross / (1 + rate).
        let rate = a.vat_rate as i64;
        let denom = 10_000 + rate;
        let base = (l.total * 10_000) / denom;
        let vat = l.total - base;
        entry.0 += base;
        entry.1 += vat;
    }
    buckets
        .into_iter()
        .map(|(rate_bp, (base, vat))| VatBucket { rate_bp, base, vat })
        .collect()
}

async fn print_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let pool = state.db.pool();
    let document = storage::get_document(pool, id).await?;
    if !document.is_closed {
        return Err(ApiError::BadRequest("document not closed".into()));
    }
    let lines = storage::list_document_details(pool, document.id).await?;
    let payments = storage::list_document_payments(pool, document.id).await?;

    let mut articles = Vec::with_capacity(lines.len());
    for l in &lines {
        articles.push(storage::get_article(pool, l.article_id).await?);
    }
    let breakdown = compute_vat_breakdown(&lines, &articles);

    let table_label = match document.table_id {
        Some(tid) => storage::get_table(pool, tid)
            .await
            .ok()
            .and_then(|t| t.name.or(Some(format!("Mesa {}", t.code))))
            .unwrap_or_else(|| "Mesa".to_string()),
        None => "Balcão".to_string(),
    };

    let payment_methods = storage::list_payment_methods(pool).await?;
    let payments_with_label: Vec<(String, i64)> = payments
        .iter()
        .map(|p| {
            let label = payment_methods
                .iter()
                .find(|m| m.id == p.payment_method_id)
                .map(|m| m.name.clone())
                .unwrap_or_else(|| "?".into());
            (label, p.amount)
        })
        .collect();

    let vat_rows: Vec<devices::escpos::VatRow> = breakdown
        .iter()
        .map(|b| devices::escpos::VatRow {
            label: format!("{:.1}%", b.rate_bp as f64 / 100.0),
            base: b.base,
            vat: b.vat,
        })
        .collect();

    let qr_block = document
        .qr_payload
        .as_deref()
        .and_then(|p| fiscal::render_qr_ascii(p).ok())
        .unwrap_or_default();

    let receipt = devices::escpos::format_legal_receipt(devices::escpos::ReceiptCtx {
        company_legal_name: &state.config.company.legal_name,
        company_trade_name: state.config.company.trade_name.as_deref(),
        company_nif: &state.config.company.nif,
        company_address: &state.config.company.address,
        company_postal_city: &format_postal(&state.config.company),
        company_share_capital_cents: state.config.company.share_capital_cents,
        company_registry: state
            .config
            .company
            .registry_office
            .as_deref()
            .zip(state.config.company.registry_number.as_deref()),
        terminal: &state.config.terminal_label,
        table_label: &table_label,
        document_type_label: "Factura Simplificada",
        document_identifier: &format!(
            "{} {}",
            document.document_type.as_deref().unwrap_or(""),
            document
                .document_number
                .map(|n| n.to_string())
                .unwrap_or_default()
        ),
        atcud: document.atcud.as_deref().unwrap_or(""),
        hash_short: document.hash_short.as_deref().unwrap_or(""),
        software_certificate: &state.config.company.software_certificate,
        issued_at: document.issued_at.unwrap_or(document.created_at),
        lines: lines
            .iter()
            .zip(articles.iter())
            .map(|(l, a)| devices::escpos::ReceiptLine {
                name: &a.name,
                qty: l.qty,
                unit_price: l.unit_price,
                total: l.total,
                vat_label: format!("{:.0}%", a.vat_rate as f64 / 100.0),
            })
            .collect(),
        vat_rows,
        total: document.total,
        payments: payments_with_label,
        qr_block: &qr_block,
        qr_payload: document.qr_payload.as_deref().unwrap_or(""),
    });

    let printer = devices::GenericPrinter::new(state.config.printer_output_path.clone());
    printer
        .print_receipt(&receipt)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, receipt))
}

fn format_postal(company: &CompanyConfig) -> String {
    match (company.postal_code.as_deref(), company.city.as_deref()) {
        (Some(pc), Some(city)) => format!("{} {}", pc, city),
        (Some(pc), None) => pc.to_string(),
        (None, Some(city)) => city.to_string(),
        (None, None) => String::new(),
    }
}
