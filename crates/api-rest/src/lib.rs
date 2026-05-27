use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use server::{AppState, CompanyConfig, SystemEvent};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use domain::{
    Article, Atcud, Document, DocumentDetail, DocumentSeries, Employee, Family, Local, MesaEstado,
    Payment, PaymentMethod, Table,
};

mod error;
use error::ApiError;

pub type ApiResult<T> = Result<T, ApiError>;

/// Tri-state for PATCH-like JSON: `Missing` (key not present), `Set(value)`.
/// `Set(None)` represents an explicit JSON `null`.
#[derive(Debug)]
pub enum OptionalField<T> {
    Missing,
    Set(T),
}

impl<T> Default for OptionalField<T> {
    fn default() -> Self {
        OptionalField::Missing
    }
}

impl<T> OptionalField<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            OptionalField::Missing => None,
            OptionalField::Set(v) => Some(v),
        }
    }
}

fn deserialize_optional_field<'de, D, T>(
    deserializer: D,
) -> Result<OptionalField<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Ok(OptionalField::Set(T::deserialize(deserializer)?))
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health))
        .route("/api/catalog", get(get_catalog))
        .route("/api/locais", get(get_locais).post(create_local))
        .route(
            "/api/locais/:id",
            get(get_local).put(update_local).delete(delete_local),
        )
        .route(
            "/api/locais/:id/tables",
            get(get_local_tables).post(create_local_table),
        )
        .route("/api/tables", get(get_tables))
        .route("/api/tables/:id", put(update_table).delete(delete_table_route))
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

#[derive(Serialize)]
pub struct TableWithEstado {
    #[serde(flatten)]
    pub table: Table,
    pub estado: MesaEstado,
}

async fn assemble_tables(
    pool: &storage::SqlitePool,
    tables: Vec<Table>,
) -> Result<Vec<TableWithEstado>, ApiError> {
    let estados = storage::list_mesa_estados(pool).await?;
    let by_id: std::collections::HashMap<Uuid, MesaEstado> =
        estados.into_iter().map(|e| (e.mesa_id, e)).collect();
    Ok(tables
        .into_iter()
        .map(|t| {
            let estado = by_id.get(&t.id).cloned().unwrap_or(MesaEstado {
                mesa_id: t.id,
                estado: domain::MesaEstadoKind::Livre,
                bloqueada_por_posto_id: None,
                bloqueada_motivo: None,
                cliente_associado_id: None,
                numero_pessoas: None,
                empregado_actual_id: None,
                aberta_em: None,
                subtotal_actual: 0,
                reservada_ate: None,
                reserva_pessoas: None,
                reserva_cliente_id: None,
                reserva_observacoes: None,
            });
            TableWithEstado { table: t, estado }
        })
        .collect())
}

async fn get_tables(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<TableWithEstado>>> {
    let tables = storage::list_tables(state.db.pool()).await?;
    Ok(Json(assemble_tables(state.db.pool(), tables).await?))
}

async fn get_locais(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Local>>> {
    Ok(Json(storage::list_locais(state.db.pool()).await?))
}

async fn get_local(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Local>> {
    Ok(Json(storage::get_local(state.db.pool(), id).await?))
}

async fn get_local_tables(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Vec<TableWithEstado>>> {
    let tables = storage::list_tables_by_local(state.db.pool(), id).await?;
    Ok(Json(assemble_tables(state.db.pool(), tables).await?))
}

#[derive(Deserialize)]
pub struct CreateLocalRequest {
    pub designacao: String,
    #[serde(default = "default_local_kind")]
    pub tipo: String,
    pub nome_generico_mesa: Option<String>,
    #[serde(default)]
    pub usa_desenho_mesas: bool,
    pub imagem: Option<String>,
    pub largura: Option<i32>,
    pub altura: Option<i32>,
}

fn default_local_kind() -> String {
    "normal".to_string()
}

async fn create_local(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateLocalRequest>,
) -> ApiResult<(StatusCode, Json<Local>)> {
    let tipo = domain::LocalKind::parse(&req.tipo)
        .ok_or_else(|| ApiError::BadRequest(format!("tipo inválido: {}", req.tipo)))?;
    let new = storage::NewLocal {
        designacao: req.designacao,
        tipo,
        nome_generico_mesa: req.nome_generico_mesa,
        usa_desenho_mesas: req.usa_desenho_mesas,
        imagem: req.imagem,
        largura: req.largura,
        altura: req.altura,
    };
    let local = storage::create_local(state.db.pool(), new).await?;
    Ok((StatusCode::CREATED, Json(local)))
}

#[derive(Deserialize, Default)]
pub struct UpdateLocalRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub designacao: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub tipo: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub nome_generico_mesa: OptionalField<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub usa_desenho_mesas: OptionalField<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub imagem: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub largura: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub altura: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub mesas_definicao: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub permite_zero_pessoas: OptionalField<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub permite_mesas_abertas_fim_do_dia: OptionalField<bool>,
}

async fn update_local(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateLocalRequest>,
) -> ApiResult<Json<Local>> {
    let tipo = match req.tipo.into_option() {
        Some(s) => Some(
            domain::LocalKind::parse(&s)
                .ok_or_else(|| ApiError::BadRequest(format!("tipo inválido: {}", s)))?,
        ),
        None => None,
    };
    let upd = storage::LocalUpdate {
        designacao: req.designacao.into_option(),
        tipo,
        nome_generico_mesa: req.nome_generico_mesa.into_option(),
        usa_desenho_mesas: req.usa_desenho_mesas.into_option(),
        imagem: req.imagem.into_option(),
        largura: req.largura.into_option(),
        altura: req.altura.into_option(),
        mesas_definicao: req.mesas_definicao.into_option(),
        permite_zero_pessoas: req.permite_zero_pessoas.into_option(),
        permite_mesas_abertas_fim_do_dia: req.permite_mesas_abertas_fim_do_dia.into_option(),
    };
    let local = storage::update_local(state.db.pool(), id, upd).await?;
    Ok(Json(local))
}

async fn delete_local(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_local(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct CreateTableRequest {
    pub code: i32,
    pub name: Option<String>,
    pub posx: Option<i32>,
    pub posy: Option<i32>,
    pub altura: Option<i32>,
    pub largura: Option<i32>,
    pub imagem: Option<String>,
}

async fn create_local_table(
    State(state): State<Arc<AppState>>,
    Path(local_id): Path<Uuid>,
    Json(req): Json<CreateTableRequest>,
) -> ApiResult<(StatusCode, Json<Table>)> {
    storage::get_local(state.db.pool(), local_id).await?;
    let new = storage::NewTable {
        local_id: Some(local_id),
        code: req.code,
        name: req.name,
        posx: req.posx,
        posy: req.posy,
        altura: req.altura,
        largura: req.largura,
        imagem: req.imagem,
    };
    let table = storage::create_table(state.db.pool(), new).await?;
    Ok((StatusCode::CREATED, Json(table)))
}

#[derive(Deserialize, Default)]
pub struct UpdateTableRequest {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub local_id: OptionalField<Option<Uuid>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub code: OptionalField<i32>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub name: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub nomeobjecto: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub posx: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub posy: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub imagem: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub fntname: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub fntsize: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub fntcolor: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub fontstyle: OptionalField<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub altura: OptionalField<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub largura: OptionalField<Option<i32>>,
}

async fn update_table(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTableRequest>,
) -> ApiResult<Json<Table>> {
    let upd = storage::TableUpdate {
        local_id: req.local_id.into_option(),
        code: req.code.into_option(),
        name: req.name.into_option(),
        nomeobjecto: req.nomeobjecto.into_option(),
        posx: req.posx.into_option(),
        posy: req.posy.into_option(),
        imagem: req.imagem.into_option(),
        fntname: req.fntname.into_option(),
        fntsize: req.fntsize.into_option(),
        fntcolor: req.fntcolor.into_option(),
        fontstyle: req.fontstyle.into_option(),
        altura: req.altura.into_option(),
        largura: req.largura.into_option(),
    };
    let table = storage::update_table(state.db.pool(), id, upd).await?;
    Ok(Json(table))
}

async fn delete_table_route(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    storage::delete_table(state.db.pool(), id).await?;
    Ok(StatusCode::NO_CONTENT)
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
