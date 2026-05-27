use chrono::{DateTime, NaiveDate, Utc};
use domain::{
    Article, Atcud, Customer, DeliveryEstado, Document, DocumentDetail, DocumentSeries, Employee,
    Family, Local, LocalKind, MesaEstado, MesaEstadoKind, Payment, PaymentMethod, PedidoDelivery,
    Table,
};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::StorageError;

fn parse_optional_uuid(s: Option<String>) -> Result<Option<Uuid>, StorageError> {
    s.map(|s| Uuid::parse_str(&s).map_err(StorageError::from))
        .transpose()
}

fn document_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Document, StorageError> {
    let table_id: Option<String> = row.try_get("table_id")?;
    let employee_id: Option<String> = row.try_get("employee_id")?;
    let series_id: Option<String> = row.try_get("series_id")?;
    let customer_id: Option<String> = row.try_get("customer_id")?;
    let local_id: Option<String> = row.try_get("local_id")?;
    Ok(Document {
        id: Uuid::parse_str(row.try_get::<&str, _>("id")?)?,
        table_id: parse_optional_uuid(table_id)?,
        employee_id: parse_optional_uuid(employee_id)?,
        total: row.try_get::<i64, _>("total")?,
        is_closed: row.try_get::<bool, _>("is_closed")?,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        series_id: parse_optional_uuid(series_id)?,
        document_type: row.try_get("document_type")?,
        document_number: row.try_get::<Option<i64>, _>("document_number")?.map(|n| n as i32),
        atcud: row.try_get("atcud")?,
        hash: row.try_get("hash")?,
        hash_short: row.try_get("hash_short")?,
        previous_hash: row.try_get("previous_hash")?,
        issued_at: row.try_get::<Option<DateTime<Utc>>, _>("issued_at")?,
        qr_payload: row.try_get("qr_payload")?,
        customer_id: parse_optional_uuid(customer_id)?,
        local_id: parse_optional_uuid(local_id)?,
        observacoes_pedido: row.try_get("observacoes_pedido")?,
        observacoes_factura: row.try_get("observacoes_factura")?,
        observacoes_cliente: row.try_get("observacoes_cliente")?,
        observacoes_morada: row.try_get("observacoes_morada")?,
        delivery_morada: row.try_get("delivery_morada")?,
        delivery_telefone: row.try_get("delivery_telefone")?,
    })
}

const DOC_COLS: &str = "id, table_id, employee_id, total, is_closed, created_at, \
        series_id, document_type, document_number, atcud, hash, hash_short, \
        previous_hash, issued_at, qr_payload, customer_id, local_id, \
        observacoes_pedido, observacoes_factura, observacoes_cliente, \
        observacoes_morada, delivery_morada, delivery_telefone";

pub async fn list_families(pool: &SqlitePool) -> Result<Vec<Family>, StorageError> {
    let rows = sqlx::query("SELECT id, parent_id, code, name FROM families ORDER BY code")
        .fetch_all(pool)
        .await?;
    rows.into_iter()
        .map(|r| {
            let parent_id: Option<String> = r.try_get("parent_id")?;
            Ok(Family {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                parent_id: parse_optional_uuid(parent_id)?,
                code: r.try_get::<i64, _>("code")? as i32,
                name: r.try_get("name")?,
            })
        })
        .collect()
}

pub async fn list_articles(pool: &SqlitePool) -> Result<Vec<Article>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, family_id, code, name, price, vat_rate, created_at, updated_at \
         FROM articles ORDER BY code",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            let family_id: Option<String> = r.try_get("family_id")?;
            Ok(Article {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                family_id: parse_optional_uuid(family_id)?,
                code: r.try_get::<i64, _>("code")? as i32,
                name: r.try_get("name")?,
                price: r.try_get::<i64, _>("price")?,
                vat_rate: r.try_get::<i64, _>("vat_rate")? as i32,
                created_at: r.try_get::<DateTime<Utc>, _>("created_at")?,
                updated_at: r.try_get::<DateTime<Utc>, _>("updated_at")?,
            })
        })
        .collect()
}

const TABLE_COLS: &str = "id, local_id, code, name, nomeobjecto, posx, posy, imagem, \
        fntname, fntsize, fntcolor, fontx, fonty, fontstyle, estadox, estadoy, \
        reservax, reservay, altura, largura, criada_em";

fn table_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Table, StorageError> {
    let local_id: Option<String> = r.try_get("local_id")?;
    Ok(Table {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        local_id: parse_optional_uuid(local_id)?,
        code: r.try_get::<i64, _>("code")? as i32,
        name: r.try_get("name")?,
        nomeobjecto: r.try_get("nomeobjecto")?,
        posx: r.try_get::<Option<i64>, _>("posx")?.map(|v| v as i32),
        posy: r.try_get::<Option<i64>, _>("posy")?.map(|v| v as i32),
        imagem: r.try_get("imagem")?,
        fntname: r.try_get("fntname")?,
        fntsize: r.try_get::<Option<i64>, _>("fntsize")?.map(|v| v as i32),
        fntcolor: r.try_get("fntcolor")?,
        fontx: r.try_get::<Option<i64>, _>("fontx")?.map(|v| v as i32),
        fonty: r.try_get::<Option<i64>, _>("fonty")?.map(|v| v as i32),
        fontstyle: r.try_get("fontstyle")?,
        estadox: r.try_get::<Option<i64>, _>("estadox")?.map(|v| v as i32),
        estadoy: r.try_get::<Option<i64>, _>("estadoy")?.map(|v| v as i32),
        reservax: r.try_get::<Option<i64>, _>("reservax")?.map(|v| v as i32),
        reservay: r.try_get::<Option<i64>, _>("reservay")?.map(|v| v as i32),
        altura: r.try_get::<Option<i64>, _>("altura")?.map(|v| v as i32),
        largura: r.try_get::<Option<i64>, _>("largura")?.map(|v| v as i32),
        criada_em: r.try_get::<Option<DateTime<Utc>>, _>("criada_em")?,
    })
}

pub async fn list_tables(pool: &SqlitePool) -> Result<Vec<Table>, StorageError> {
    let q = format!("SELECT {TABLE_COLS} FROM tables ORDER BY code");
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(table_from_row).collect()
}

pub async fn list_tables_by_local(
    pool: &SqlitePool,
    local_id: Uuid,
) -> Result<Vec<Table>, StorageError> {
    let q = format!("SELECT {TABLE_COLS} FROM tables WHERE local_id = ?1 ORDER BY code");
    let rows = sqlx::query(&q)
        .bind(local_id.to_string())
        .fetch_all(pool)
        .await?;
    rows.iter().map(table_from_row).collect()
}

pub async fn list_employees(pool: &SqlitePool) -> Result<Vec<Employee>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, code, name, perc_consumo, base_consumo FROM employees ORDER BY code",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            Ok(Employee {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                code: r.try_get::<i64, _>("code")? as i32,
                name: r.try_get("name")?,
                perc_consumo: r.try_get::<i64, _>("perc_consumo")? as i32,
                base_consumo: r.try_get::<i64, _>("base_consumo")?,
            })
        })
        .collect()
}

pub async fn get_employee(pool: &SqlitePool, id: Uuid) -> Result<Employee, StorageError> {
    let r = sqlx::query(
        "SELECT id, code, name, perc_consumo, base_consumo FROM employees WHERE id = ?1",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?
    .ok_or(StorageError::NotFound)?;
    Ok(Employee {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        code: r.try_get::<i64, _>("code")? as i32,
        name: r.try_get("name")?,
        perc_consumo: r.try_get::<i64, _>("perc_consumo")? as i32,
        base_consumo: r.try_get::<i64, _>("base_consumo")?,
    })
}

pub async fn list_payment_methods(pool: &SqlitePool) -> Result<Vec<PaymentMethod>, StorageError> {
    let rows = sqlx::query("SELECT id, code, name FROM payment_methods ORDER BY code")
        .fetch_all(pool)
        .await?;
    rows.into_iter()
        .map(|r| {
            Ok(PaymentMethod {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                code: r.try_get::<i64, _>("code")? as i32,
                name: r.try_get("name")?,
            })
        })
        .collect()
}

pub async fn get_article(pool: &SqlitePool, id: Uuid) -> Result<Article, StorageError> {
    let row = sqlx::query(
        "SELECT id, family_id, code, name, price, vat_rate, created_at, updated_at \
         FROM articles WHERE id = ?1",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?
    .ok_or(StorageError::NotFound)?;

    let family_id: Option<String> = row.try_get("family_id")?;
    Ok(Article {
        id: Uuid::parse_str(row.try_get::<&str, _>("id")?)?,
        family_id: parse_optional_uuid(family_id)?,
        code: row.try_get::<i64, _>("code")? as i32,
        name: row.try_get("name")?,
        price: row.try_get::<i64, _>("price")?,
        vat_rate: row.try_get::<i64, _>("vat_rate")? as i32,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at")?,
        updated_at: row.try_get::<DateTime<Utc>, _>("updated_at")?,
    })
}

pub async fn get_table(pool: &SqlitePool, id: Uuid) -> Result<Table, StorageError> {
    let q = format!("SELECT {TABLE_COLS} FROM tables WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    table_from_row(&row)
}

fn mesa_estado_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<MesaEstado, StorageError> {
    let parse_uuid = |key: &str| -> Result<Option<Uuid>, StorageError> {
        let s: Option<String> = r.try_get(key)?;
        parse_optional_uuid(s)
    };
    let estado_s: String = r.try_get("estado")?;
    let estado = MesaEstadoKind::parse(&estado_s).ok_or(StorageError::NotFound)?;
    Ok(MesaEstado {
        mesa_id: Uuid::parse_str(r.try_get::<&str, _>("mesa_id")?)?,
        estado,
        bloqueada_por_posto_id: parse_uuid("bloqueada_por_posto_id")?,
        bloqueada_motivo: r.try_get("bloqueada_motivo")?,
        cliente_associado_id: parse_uuid("cliente_associado_id")?,
        numero_pessoas: r.try_get::<Option<i64>, _>("numero_pessoas")?.map(|v| v as i32),
        empregado_actual_id: parse_uuid("empregado_actual_id")?,
        aberta_em: r.try_get::<Option<DateTime<Utc>>, _>("aberta_em")?,
        subtotal_actual: r.try_get::<i64, _>("subtotal_actual")?,
        reservada_ate: r.try_get::<Option<DateTime<Utc>>, _>("reservada_ate")?,
        reserva_pessoas: r.try_get::<Option<i64>, _>("reserva_pessoas")?.map(|v| v as i32),
        reserva_cliente_id: parse_uuid("reserva_cliente_id")?,
        reserva_observacoes: r.try_get("reserva_observacoes")?,
    })
}

const ESTADO_COLS: &str = "mesa_id, estado, bloqueada_por_posto_id, bloqueada_motivo, \
        cliente_associado_id, numero_pessoas, empregado_actual_id, aberta_em, subtotal_actual, \
        reservada_ate, reserva_pessoas, reserva_cliente_id, reserva_observacoes";

pub async fn list_mesa_estados(pool: &SqlitePool) -> Result<Vec<MesaEstado>, StorageError> {
    let q = format!("SELECT {ESTADO_COLS} FROM mesa_estado");
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(mesa_estado_from_row).collect()
}

pub async fn get_mesa_estado(
    pool: &SqlitePool,
    mesa_id: Uuid,
) -> Result<MesaEstado, StorageError> {
    let q = format!("SELECT {ESTADO_COLS} FROM mesa_estado WHERE mesa_id = ?1");
    if let Some(row) = sqlx::query(&q)
        .bind(mesa_id.to_string())
        .fetch_optional(pool)
        .await?
    {
        return mesa_estado_from_row(&row);
    }
    Ok(MesaEstado {
        mesa_id,
        estado: MesaEstadoKind::Livre,
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
    })
}

async fn upsert_mesa_estado<'e, E>(
    executor: E,
    mesa_id: Uuid,
    estado: MesaEstadoKind,
    empregado_actual_id: Option<Uuid>,
    aberta_em: Option<DateTime<Utc>>,
    subtotal_actual: i64,
) -> Result<(), StorageError>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    sqlx::query(
        "INSERT INTO mesa_estado (mesa_id, estado, empregado_actual_id, aberta_em, subtotal_actual) \
         VALUES (?1, ?2, ?3, ?4, ?5) \
         ON CONFLICT(mesa_id) DO UPDATE SET \
            estado = excluded.estado, \
            empregado_actual_id = excluded.empregado_actual_id, \
            aberta_em = excluded.aberta_em, \
            subtotal_actual = excluded.subtotal_actual",
    )
    .bind(mesa_id.to_string())
    .bind(estado.as_str())
    .bind(empregado_actual_id.map(|u| u.to_string()))
    .bind(aberta_em)
    .bind(subtotal_actual)
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn get_open_document_for_table(
    pool: &SqlitePool,
    table_id: Uuid,
) -> Result<Option<Document>, StorageError> {
    let q = format!(
        "SELECT {DOC_COLS} FROM documents WHERE table_id = ?1 AND is_closed = 0 \
         ORDER BY created_at DESC LIMIT 1"
    );
    let row = sqlx::query(&q)
        .bind(table_id.to_string())
        .fetch_optional(pool)
        .await?;
    row.as_ref().map(document_from_row).transpose()
}

pub async fn get_document(pool: &SqlitePool, id: Uuid) -> Result<Document, StorageError> {
    let q = format!("SELECT {DOC_COLS} FROM documents WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    document_from_row(&row)
}

pub async fn list_document_details(
    pool: &SqlitePool,
    document_id: Uuid,
) -> Result<Vec<DocumentDetail>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, document_id, article_id, qty, unit_price, total \
         FROM document_details WHERE document_id = ?1 ORDER BY rowid",
    )
    .bind(document_id.to_string())
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            Ok(DocumentDetail {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                document_id: Uuid::parse_str(r.try_get::<&str, _>("document_id")?)?,
                article_id: Uuid::parse_str(r.try_get::<&str, _>("article_id")?)?,
                qty: r.try_get::<i64, _>("qty")? as i32,
                unit_price: r.try_get::<i64, _>("unit_price")?,
                total: r.try_get::<i64, _>("total")?,
            })
        })
        .collect()
}

pub async fn open_table(
    pool: &SqlitePool,
    table_id: Uuid,
    employee_id: Option<Uuid>,
) -> Result<Document, StorageError> {
    if let Some(existing) = get_open_document_for_table(pool, table_id).await? {
        return Ok(existing);
    }

    let table = get_table(pool, table_id).await?;
    let mut tx = pool.begin().await?;
    let doc_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO documents (id, table_id, employee_id, total, is_closed, created_at, local_id) \
         VALUES (?1, ?2, ?3, 0, 0, ?4, ?5)",
    )
    .bind(doc_id.to_string())
    .bind(table_id.to_string())
    .bind(employee_id.map(|i| i.to_string()))
    .bind(now)
    .bind(table.local_id.map(|u| u.to_string()))
    .execute(&mut *tx)
    .await?;

    upsert_mesa_estado(
        &mut *tx,
        table_id,
        MesaEstadoKind::Aberta,
        employee_id,
        Some(now),
        0,
    )
    .await?;

    tx.commit().await?;

    Ok(Document {
        id: doc_id,
        table_id: Some(table_id),
        employee_id,
        total: 0,
        is_closed: false,
        created_at: now,
        series_id: None,
        document_type: None,
        document_number: None,
        atcud: None,
        hash: None,
        hash_short: None,
        previous_hash: None,
        issued_at: None,
        qr_payload: None,
        customer_id: None,
        local_id: table.local_id,
        observacoes_pedido: None,
        observacoes_factura: None,
        observacoes_cliente: None,
        observacoes_morada: None,
        delivery_morada: None,
        delivery_telefone: None,
    })
}

/// Cria um documento solto, ligado apenas a um local (sem mesa física).
/// Usado pelos modos take_away (balcão) e delivery (encomenda nova).
pub async fn start_document_for_local(
    pool: &SqlitePool,
    local_id: Uuid,
    employee_id: Option<Uuid>,
) -> Result<Document, StorageError> {
    let doc_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO documents (id, table_id, employee_id, total, is_closed, created_at, \
         local_id) VALUES (?1, NULL, ?2, 0, 0, ?3, ?4)",
    )
    .bind(doc_id.to_string())
    .bind(employee_id.map(|i| i.to_string()))
    .bind(now)
    .bind(local_id.to_string())
    .execute(pool)
    .await?;
    get_document(pool, doc_id).await
}

/// Garante uma mesa "virtual" de consumo próprio para o par (local, empregado).
pub async fn ensure_consumo_table(
    pool: &SqlitePool,
    local_id: Uuid,
    employee: &Employee,
) -> Result<Table, StorageError> {
    let row = sqlx::query("SELECT id FROM tables WHERE local_id = ?1 AND code = ?2")
        .bind(local_id.to_string())
        .bind(9000 + employee.code as i64)
        .fetch_optional(pool)
        .await?;
    if let Some(r) = row {
        return get_table(pool, Uuid::parse_str(r.try_get::<&str, _>("id")?)?).await;
    }
    let new = NewTable {
        local_id: Some(local_id),
        code: 9000 + employee.code,
        name: Some(format!("Consumo {}", employee.name)),
        ..Default::default()
    };
    create_table(pool, new).await
}

#[derive(Default)]
pub struct DocumentContextUpdate {
    pub customer_id: Option<Option<Uuid>>,
    pub observacoes_pedido: Option<Option<String>>,
    pub observacoes_factura: Option<Option<String>>,
    pub observacoes_cliente: Option<Option<String>>,
    pub observacoes_morada: Option<Option<String>>,
    pub delivery_morada: Option<Option<String>>,
    pub delivery_telefone: Option<Option<String>>,
}

pub async fn update_document_context(
    pool: &SqlitePool,
    document_id: Uuid,
    upd: DocumentContextUpdate,
) -> Result<Document, StorageError> {
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE documents SET ");
    let mut first = true;
    macro_rules! push_field {
        ($field:ident, $col:literal) => {
            if let Some(v) = upd.$field {
                if !first { q.push(", "); }
                q.push($col).push(" = ");
                q.push_bind(v);
                first = false;
            }
        };
    }
    if let Some(v) = upd.customer_id {
        if !first { q.push(", "); }
        q.push("customer_id = ");
        q.push_bind(v.map(|u| u.to_string()));
        first = false;
    }
    push_field!(observacoes_pedido, "observacoes_pedido");
    push_field!(observacoes_factura, "observacoes_factura");
    push_field!(observacoes_cliente, "observacoes_cliente");
    push_field!(observacoes_morada, "observacoes_morada");
    push_field!(delivery_morada, "delivery_morada");
    push_field!(delivery_telefone, "delivery_telefone");

    if first {
        return get_document(pool, document_id).await;
    }
    q.push(" WHERE id = ").push_bind(document_id.to_string());
    q.build().execute(pool).await?;
    get_document(pool, document_id).await
}

pub async fn add_document_line(
    pool: &SqlitePool,
    document_id: Uuid,
    article_id: Uuid,
    qty: i32,
    unit_price: i64,
) -> Result<DocumentDetail, StorageError> {
    let total = unit_price * qty as i64;
    let detail_id = Uuid::new_v4();

    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT INTO document_details (id, document_id, article_id, qty, unit_price, total) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )
    .bind(detail_id.to_string())
    .bind(document_id.to_string())
    .bind(article_id.to_string())
    .bind(qty as i64)
    .bind(unit_price)
    .bind(total)
    .execute(&mut *tx)
    .await?;

    sqlx::query("UPDATE documents SET total = total + ?1 WHERE id = ?2")
        .bind(total)
        .bind(document_id.to_string())
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "UPDATE mesa_estado SET subtotal_actual = subtotal_actual + ?1 \
         WHERE mesa_id = (SELECT table_id FROM documents WHERE id = ?2)",
    )
    .bind(total)
    .bind(document_id.to_string())
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(DocumentDetail {
        id: detail_id,
        document_id,
        article_id,
        qty,
        unit_price,
        total,
    })
}

pub async fn list_document_payments(
    pool: &SqlitePool,
    document_id: Uuid,
) -> Result<Vec<Payment>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, document_id, payment_method_id, amount, created_at \
         FROM payments WHERE document_id = ?1 ORDER BY created_at",
    )
    .bind(document_id.to_string())
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            Ok(Payment {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                document_id: Uuid::parse_str(r.try_get::<&str, _>("document_id")?)?,
                payment_method_id: Uuid::parse_str(r.try_get::<&str, _>("payment_method_id")?)?,
                amount: r.try_get::<i64, _>("amount")?,
                created_at: r.try_get::<DateTime<Utc>, _>("created_at")?,
            })
        })
        .collect()
}

pub async fn record_payment(
    pool: &SqlitePool,
    document_id: Uuid,
    payment_method_id: Uuid,
    amount: i64,
) -> Result<Payment, StorageError> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO payments (id, document_id, payment_method_id, amount, created_at) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(id.to_string())
    .bind(document_id.to_string())
    .bind(payment_method_id.to_string())
    .bind(amount)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(Payment {
        id,
        document_id,
        payment_method_id,
        amount,
        created_at: now,
    })
}

/// Locks the series row, allocates the next sequential number, fetches the
/// active ATCUD validation code, and bumps the counter. Used during fiscal
/// close.
pub async fn allocate_series_number(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    document_type: &str,
    year: i32,
) -> Result<(DocumentSeries, Atcud, i32), StorageError> {
    let row = sqlx::query(
        "SELECT id, document_type, prefix, year, next_number, is_active \
         FROM document_series WHERE document_type = ?1 AND year = ?2 AND is_active = 1 \
         ORDER BY prefix LIMIT 1",
    )
    .bind(document_type)
    .bind(year as i64)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or(StorageError::NotFound)?;

    let series = DocumentSeries {
        id: Uuid::parse_str(row.try_get::<&str, _>("id")?)?,
        document_type: row.try_get("document_type")?,
        prefix: row.try_get("prefix")?,
        year: row.try_get::<i64, _>("year")? as i32,
        next_number: row.try_get::<i64, _>("next_number")? as i32,
        is_active: row.try_get::<bool, _>("is_active")?,
    };

    let atcud_row = sqlx::query(
        "SELECT id, document_type, series_prefix, year, atcud, start_date, registered_at, is_active \
         FROM atcud WHERE document_type = ?1 AND series_prefix = ?2 AND year = ?3 AND is_active = 1 \
         ORDER BY registered_at DESC LIMIT 1",
    )
    .bind(&series.document_type)
    .bind(&series.prefix)
    .bind(series.year as i64)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or(StorageError::NotFound)?;

    let atcud = Atcud {
        id: Uuid::parse_str(atcud_row.try_get::<&str, _>("id")?)?,
        document_type: atcud_row.try_get("document_type")?,
        series_prefix: atcud_row.try_get("series_prefix")?,
        year: atcud_row.try_get::<i64, _>("year")? as i32,
        atcud: atcud_row.try_get("atcud")?,
        start_date: atcud_row.try_get::<NaiveDate, _>("start_date")?,
        registered_at: atcud_row.try_get::<DateTime<Utc>, _>("registered_at")?,
        is_active: atcud_row.try_get::<bool, _>("is_active")?,
    };

    let allocated = series.next_number;
    sqlx::query("UPDATE document_series SET next_number = next_number + 1 WHERE id = ?1")
        .bind(series.id.to_string())
        .execute(&mut **tx)
        .await?;

    Ok((series, atcud, allocated))
}

/// Reads the latest closed document's hash for this `series_id` (chain head).
pub async fn last_hash_for_series(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    series_id: Uuid,
) -> Result<Option<String>, StorageError> {
    let row = sqlx::query(
        "SELECT hash FROM documents WHERE series_id = ?1 AND hash IS NOT NULL \
         ORDER BY document_number DESC LIMIT 1",
    )
    .bind(series_id.to_string())
    .fetch_optional(&mut **tx)
    .await?;
    Ok(row.and_then(|r| r.try_get::<Option<String>, _>("hash").ok().flatten()))
}

#[allow(clippy::too_many_arguments)]
pub async fn finalize_document_fiscal(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    document_id: Uuid,
    series_id: Uuid,
    document_type: &str,
    document_number: i32,
    atcud: &str,
    hash: &str,
    hash_short: &str,
    previous_hash: &str,
    issued_at: DateTime<Utc>,
    qr_payload: &str,
) -> Result<(), StorageError> {
    sqlx::query(
        "UPDATE documents SET series_id = ?1, document_type = ?2, document_number = ?3, \
         atcud = ?4, hash = ?5, hash_short = ?6, previous_hash = ?7, issued_at = ?8, \
         qr_payload = ?9, is_closed = 1 WHERE id = ?10",
    )
    .bind(series_id.to_string())
    .bind(document_type)
    .bind(document_number as i64)
    .bind(atcud)
    .bind(hash)
    .bind(hash_short)
    .bind(previous_hash)
    .bind(issued_at)
    .bind(qr_payload)
    .bind(document_id.to_string())
    .execute(&mut **tx)
    .await?;

    if let Some(tid) = sqlx::query("SELECT table_id FROM documents WHERE id = ?1")
        .bind(document_id.to_string())
        .fetch_one(&mut **tx)
        .await?
        .try_get::<Option<String>, _>("table_id")?
    {
        sqlx::query(
            "UPDATE mesa_estado SET estado = 'livre', empregado_actual_id = NULL, \
             aberta_em = NULL, subtotal_actual = 0 WHERE mesa_id = ?1",
        )
        .bind(tid)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

pub async fn list_series(pool: &SqlitePool) -> Result<Vec<DocumentSeries>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, document_type, prefix, year, next_number, is_active \
         FROM document_series ORDER BY year, document_type, prefix",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            Ok(DocumentSeries {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                document_type: r.try_get("document_type")?,
                prefix: r.try_get("prefix")?,
                year: r.try_get::<i64, _>("year")? as i32,
                next_number: r.try_get::<i64, _>("next_number")? as i32,
                is_active: r.try_get::<bool, _>("is_active")?,
            })
        })
        .collect()
}

const LOCAL_COLS: &str = "id, designacao, mesas_definicao, tipo, tipo_preco_id, \
        metodo_pagamento_default_id, taxa_servico_artigo_id, limite_consumo, \
        imprime_conta_acima_de, nome_generico_mesa, imprime_subtotal_em, imprime_conta_em, \
        fecha_mesa_ao_pedir, usa_iva_venda_directa, iva_excluido_dos_precos, \
        cor_empregado_na_lista, impressora_directa_pedidos_id, pede_nova_mesa_depois_de_fechar, \
        pede_nova_mesa_apos_pedido, indica_pessoas_obrigatorio, indica_pessoas_apenas_abertura, \
        permite_zero_pessoas, aloca_mesas_dinamicamente, alocacao_circular, \
        inclui_desconto_nos_precos, artigos_automatico_sem_preco, carregamento_rapido_mesas, \
        so_imprime_pedidos_com_complementos, lista_grande_pedidos, mesas_uma_vez_por_dia, \
        facturacao_externa, nao_agrupa_detalhes_na_conta, permite_encaixe_promocoes, \
        separa_artigos_antes_encaixe, permite_mesas_abertas_fim_do_dia, \
        pode_identificar_cliente_no_pedido, obriga_indicar_valor_pago, usa_desenho_mesas, \
        imagem, largura, altura, anulado_em";

fn local_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Local, StorageError> {
    let parse_uuid_field = |key: &str| -> Result<Option<Uuid>, StorageError> {
        let s: Option<String> = r.try_get(key)?;
        parse_optional_uuid(s)
    };
    let tipo_s: String = r.try_get("tipo")?;
    let tipo = LocalKind::parse(&tipo_s).ok_or(StorageError::NotFound)?;
    let subtotal_json: String = r.try_get("imprime_subtotal_em")?;
    let conta_json: String = r.try_get("imprime_conta_em")?;
    Ok(Local {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        designacao: r.try_get("designacao")?,
        mesas_definicao: r.try_get("mesas_definicao")?,
        tipo,
        tipo_preco_id: parse_uuid_field("tipo_preco_id")?,
        metodo_pagamento_default_id: parse_uuid_field("metodo_pagamento_default_id")?,
        taxa_servico_artigo_id: parse_uuid_field("taxa_servico_artigo_id")?,
        limite_consumo: r.try_get::<i64, _>("limite_consumo")?,
        imprime_conta_acima_de: r.try_get::<i64, _>("imprime_conta_acima_de")?,
        nome_generico_mesa: r.try_get("nome_generico_mesa")?,
        imprime_subtotal_em: serde_json::from_str(&subtotal_json).unwrap_or(serde_json::json!({})),
        imprime_conta_em: serde_json::from_str(&conta_json).unwrap_or(serde_json::json!({})),
        fecha_mesa_ao_pedir: r.try_get("fecha_mesa_ao_pedir")?,
        usa_iva_venda_directa: r.try_get::<bool, _>("usa_iva_venda_directa")?,
        iva_excluido_dos_precos: r.try_get::<bool, _>("iva_excluido_dos_precos")?,
        cor_empregado_na_lista: r.try_get::<bool, _>("cor_empregado_na_lista")?,
        impressora_directa_pedidos_id: parse_uuid_field("impressora_directa_pedidos_id")?,
        pede_nova_mesa_depois_de_fechar: r.try_get::<bool, _>("pede_nova_mesa_depois_de_fechar")?,
        pede_nova_mesa_apos_pedido: r.try_get::<bool, _>("pede_nova_mesa_apos_pedido")?,
        indica_pessoas_obrigatorio: r.try_get::<bool, _>("indica_pessoas_obrigatorio")?,
        indica_pessoas_apenas_abertura: r.try_get::<bool, _>("indica_pessoas_apenas_abertura")?,
        permite_zero_pessoas: r.try_get::<bool, _>("permite_zero_pessoas")?,
        aloca_mesas_dinamicamente: r.try_get::<bool, _>("aloca_mesas_dinamicamente")?,
        alocacao_circular: r.try_get::<bool, _>("alocacao_circular")?,
        inclui_desconto_nos_precos: r.try_get::<bool, _>("inclui_desconto_nos_precos")?,
        artigos_automatico_sem_preco: r.try_get::<bool, _>("artigos_automatico_sem_preco")?,
        carregamento_rapido_mesas: r.try_get::<bool, _>("carregamento_rapido_mesas")?,
        so_imprime_pedidos_com_complementos: r
            .try_get::<bool, _>("so_imprime_pedidos_com_complementos")?,
        lista_grande_pedidos: r.try_get::<bool, _>("lista_grande_pedidos")?,
        mesas_uma_vez_por_dia: r.try_get::<bool, _>("mesas_uma_vez_por_dia")?,
        facturacao_externa: r.try_get::<bool, _>("facturacao_externa")?,
        nao_agrupa_detalhes_na_conta: r.try_get::<bool, _>("nao_agrupa_detalhes_na_conta")?,
        permite_encaixe_promocoes: r.try_get::<bool, _>("permite_encaixe_promocoes")?,
        separa_artigos_antes_encaixe: r.try_get::<bool, _>("separa_artigos_antes_encaixe")?,
        permite_mesas_abertas_fim_do_dia: r.try_get::<bool, _>("permite_mesas_abertas_fim_do_dia")?,
        pode_identificar_cliente_no_pedido: r
            .try_get::<bool, _>("pode_identificar_cliente_no_pedido")?,
        obriga_indicar_valor_pago: r.try_get::<bool, _>("obriga_indicar_valor_pago")?,
        usa_desenho_mesas: r.try_get::<bool, _>("usa_desenho_mesas")?,
        imagem: r.try_get("imagem")?,
        largura: r.try_get::<Option<i64>, _>("largura")?.map(|v| v as i32),
        altura: r.try_get::<Option<i64>, _>("altura")?.map(|v| v as i32),
        anulado_em: r.try_get::<Option<DateTime<Utc>>, _>("anulado_em")?,
    })
}

pub async fn list_locais(pool: &SqlitePool) -> Result<Vec<Local>, StorageError> {
    let q = format!(
        "SELECT {LOCAL_COLS} FROM locais WHERE anulado_em IS NULL ORDER BY designacao"
    );
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(local_from_row).collect()
}

pub async fn get_local(pool: &SqlitePool, id: Uuid) -> Result<Local, StorageError> {
    let q = format!("SELECT {LOCAL_COLS} FROM locais WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    local_from_row(&row)
}

pub struct NewLocal {
    pub designacao: String,
    pub tipo: LocalKind,
    pub nome_generico_mesa: Option<String>,
    pub usa_desenho_mesas: bool,
    pub imagem: Option<String>,
    pub largura: Option<i32>,
    pub altura: Option<i32>,
}

pub async fn create_local(pool: &SqlitePool, input: NewLocal) -> Result<Local, StorageError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO locais (id, designacao, tipo, nome_generico_mesa, usa_desenho_mesas, \
         imagem, largura, altura) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )
    .bind(id.to_string())
    .bind(&input.designacao)
    .bind(input.tipo.as_str())
    .bind(
        input
            .nome_generico_mesa
            .unwrap_or_else(|| "Mesa {nm}".to_string()),
    )
    .bind(input.usa_desenho_mesas as i32)
    .bind(input.imagem.as_deref())
    .bind(input.largura.map(|v| v as i64))
    .bind(input.altura.map(|v| v as i64))
    .execute(pool)
    .await?;
    get_local(pool, id).await
}

#[derive(Default)]
pub struct LocalUpdate {
    pub designacao: Option<String>,
    pub tipo: Option<LocalKind>,
    pub nome_generico_mesa: Option<String>,
    pub usa_desenho_mesas: Option<bool>,
    pub imagem: Option<Option<String>>,
    pub largura: Option<Option<i32>>,
    pub altura: Option<Option<i32>>,
    pub mesas_definicao: Option<Option<String>>,
    pub permite_zero_pessoas: Option<bool>,
    pub permite_mesas_abertas_fim_do_dia: Option<bool>,
}

pub async fn update_local(
    pool: &SqlitePool,
    id: Uuid,
    upd: LocalUpdate,
) -> Result<Local, StorageError> {
    let mut sets: Vec<String> = Vec::new();
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE locais SET ");
    let mut first = true;
    let push = |q: &mut sqlx::QueryBuilder<sqlx::Sqlite>,
                sets: &mut Vec<String>,
                first: &mut bool,
                col: &str| {
        if !*first {
            q.push(", ");
        }
        q.push(col).push(" = ");
        *first = false;
        sets.push(col.to_string());
    };
    if let Some(v) = upd.designacao {
        push(&mut q, &mut sets, &mut first, "designacao");
        q.push_bind(v);
    }
    if let Some(v) = upd.tipo {
        push(&mut q, &mut sets, &mut first, "tipo");
        q.push_bind(v.as_str().to_string());
    }
    if let Some(v) = upd.nome_generico_mesa {
        push(&mut q, &mut sets, &mut first, "nome_generico_mesa");
        q.push_bind(v);
    }
    if let Some(v) = upd.usa_desenho_mesas {
        push(&mut q, &mut sets, &mut first, "usa_desenho_mesas");
        q.push_bind(v as i32);
    }
    if let Some(v) = upd.imagem {
        push(&mut q, &mut sets, &mut first, "imagem");
        q.push_bind(v);
    }
    if let Some(v) = upd.largura {
        push(&mut q, &mut sets, &mut first, "largura");
        q.push_bind(v.map(|x| x as i64));
    }
    if let Some(v) = upd.altura {
        push(&mut q, &mut sets, &mut first, "altura");
        q.push_bind(v.map(|x| x as i64));
    }
    if let Some(v) = upd.mesas_definicao {
        push(&mut q, &mut sets, &mut first, "mesas_definicao");
        q.push_bind(v);
    }
    if let Some(v) = upd.permite_zero_pessoas {
        push(&mut q, &mut sets, &mut first, "permite_zero_pessoas");
        q.push_bind(v as i32);
    }
    if let Some(v) = upd.permite_mesas_abertas_fim_do_dia {
        push(&mut q, &mut sets, &mut first, "permite_mesas_abertas_fim_do_dia");
        q.push_bind(v as i32);
    }

    if sets.is_empty() {
        return get_local(pool, id).await;
    }

    q.push(" WHERE id = ").push_bind(id.to_string());
    q.build().execute(pool).await?;
    get_local(pool, id).await
}

pub async fn delete_local(pool: &SqlitePool, id: Uuid) -> Result<(), StorageError> {
    sqlx::query("UPDATE locais SET anulado_em = ?1 WHERE id = ?2")
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(Default)]
pub struct NewTable {
    pub local_id: Option<Uuid>,
    pub code: i32,
    pub name: Option<String>,
    pub posx: Option<i32>,
    pub posy: Option<i32>,
    pub altura: Option<i32>,
    pub largura: Option<i32>,
    pub imagem: Option<String>,
}

pub async fn create_table(pool: &SqlitePool, input: NewTable) -> Result<Table, StorageError> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO tables (id, local_id, code, name, posx, posy, altura, largura, imagem, \
         criada_em) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
    )
    .bind(id.to_string())
    .bind(input.local_id.map(|u| u.to_string()))
    .bind(input.code as i64)
    .bind(input.name.as_deref())
    .bind(input.posx.map(|v| v as i64))
    .bind(input.posy.map(|v| v as i64))
    .bind(input.altura.map(|v| v as i64))
    .bind(input.largura.map(|v| v as i64))
    .bind(input.imagem.as_deref())
    .bind(now)
    .execute(pool)
    .await?;
    get_table(pool, id).await
}

#[derive(Default)]
pub struct TableUpdate {
    pub local_id: Option<Option<Uuid>>,
    pub code: Option<i32>,
    pub name: Option<Option<String>>,
    pub nomeobjecto: Option<Option<String>>,
    pub posx: Option<Option<i32>>,
    pub posy: Option<Option<i32>>,
    pub imagem: Option<Option<String>>,
    pub fntname: Option<Option<String>>,
    pub fntsize: Option<Option<i32>>,
    pub fntcolor: Option<Option<String>>,
    pub fontstyle: Option<Option<String>>,
    pub altura: Option<Option<i32>>,
    pub largura: Option<Option<i32>>,
}

pub async fn update_table(
    pool: &SqlitePool,
    id: Uuid,
    upd: TableUpdate,
) -> Result<Table, StorageError> {
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE tables SET ");
    let mut first = true;
    macro_rules! set_int {
        ($field:ident, $col:literal) => {
            if let Some(v) = upd.$field {
                if !first { q.push(", "); }
                q.push($col).push(" = ");
                q.push_bind(v.map(|x| x as i64));
                first = false;
            }
        };
    }
    macro_rules! set_string {
        ($field:ident, $col:literal) => {
            if let Some(v) = upd.$field {
                if !first { q.push(", "); }
                q.push($col).push(" = ");
                q.push_bind(v);
                first = false;
            }
        };
    }

    if let Some(v) = upd.local_id {
        if !first { q.push(", "); }
        q.push("local_id = ");
        q.push_bind(v.map(|u| u.to_string()));
        first = false;
    }
    if let Some(v) = upd.code {
        if !first { q.push(", "); }
        q.push("code = ");
        q.push_bind(v as i64);
        first = false;
    }
    set_string!(name, "name");
    set_string!(nomeobjecto, "nomeobjecto");
    set_int!(posx, "posx");
    set_int!(posy, "posy");
    set_string!(imagem, "imagem");
    set_string!(fntname, "fntname");
    set_int!(fntsize, "fntsize");
    set_string!(fntcolor, "fntcolor");
    set_string!(fontstyle, "fontstyle");
    set_int!(altura, "altura");
    set_int!(largura, "largura");

    if first {
        return get_table(pool, id).await;
    }
    q.push(" WHERE id = ").push_bind(id.to_string());
    q.build().execute(pool).await?;
    get_table(pool, id).await
}

pub async fn delete_table(pool: &SqlitePool, id: Uuid) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM mesa_estado WHERE mesa_id = ?1")
        .bind(id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM tables WHERE id = ?1")
        .bind(id.to_string())
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

const CUSTOMER_COLS: &str = "id, codigo, nome, nif, telefone, morada, cod_postal, localidade, \
        email, observacoes, numero_cartao, limite_credito, zona_id, anulado_em";

fn customer_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Customer, StorageError> {
    let zona_id: Option<String> = r.try_get("zona_id")?;
    Ok(Customer {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        codigo: r.try_get::<Option<i64>, _>("codigo")?.map(|v| v as i32),
        nome: r.try_get("nome")?,
        nif: r.try_get("nif")?,
        telefone: r.try_get("telefone")?,
        morada: r.try_get("morada")?,
        cod_postal: r.try_get("cod_postal")?,
        localidade: r.try_get("localidade")?,
        email: r.try_get("email")?,
        observacoes: r.try_get("observacoes")?,
        numero_cartao: r.try_get("numero_cartao")?,
        limite_credito: r.try_get::<i64, _>("limite_credito")?,
        zona_id: parse_optional_uuid(zona_id)?,
        anulado_em: r.try_get::<Option<DateTime<Utc>>, _>("anulado_em")?,
    })
}

pub async fn list_customers(pool: &SqlitePool) -> Result<Vec<Customer>, StorageError> {
    let q = format!(
        "SELECT {CUSTOMER_COLS} FROM clientes WHERE anulado_em IS NULL ORDER BY nome"
    );
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(customer_from_row).collect()
}

pub async fn get_customer(pool: &SqlitePool, id: Uuid) -> Result<Customer, StorageError> {
    let q = format!("SELECT {CUSTOMER_COLS} FROM clientes WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    customer_from_row(&row)
}

/// Procura clientes por telefone (sufixo) ou nome (substring case-insensitive).
pub async fn search_customers(
    pool: &SqlitePool,
    phone: Option<&str>,
    name: Option<&str>,
) -> Result<Vec<Customer>, StorageError> {
    if phone.is_none() && name.is_none() {
        return list_customers(pool).await;
    }
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new(
        format!("SELECT {CUSTOMER_COLS} FROM clientes WHERE anulado_em IS NULL"),
    );
    if let Some(p) = phone {
        q.push(" AND telefone LIKE ");
        q.push_bind(format!("%{}", p));
    }
    if let Some(n) = name {
        q.push(" AND LOWER(nome) LIKE ");
        q.push_bind(format!("%{}%", n.to_lowercase()));
    }
    q.push(" ORDER BY nome LIMIT 30");
    let rows = q.build().fetch_all(pool).await?;
    rows.iter().map(customer_from_row).collect()
}

pub struct NewCustomer {
    pub nome: String,
    pub nif: Option<String>,
    pub telefone: Option<String>,
    pub morada: Option<String>,
    pub cod_postal: Option<String>,
    pub localidade: Option<String>,
    pub email: Option<String>,
    pub observacoes: Option<String>,
}

pub async fn create_customer(
    pool: &SqlitePool,
    input: NewCustomer,
) -> Result<Customer, StorageError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO clientes (id, nome, nif, telefone, morada, cod_postal, localidade, email, \
         observacoes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )
    .bind(id.to_string())
    .bind(&input.nome)
    .bind(input.nif.as_deref())
    .bind(input.telefone.as_deref())
    .bind(input.morada.as_deref())
    .bind(input.cod_postal.as_deref())
    .bind(input.localidade.as_deref())
    .bind(input.email.as_deref())
    .bind(input.observacoes.as_deref())
    .execute(pool)
    .await?;
    get_customer(pool, id).await
}

#[derive(Default)]
pub struct CustomerUpdate {
    pub nome: Option<String>,
    pub nif: Option<Option<String>>,
    pub telefone: Option<Option<String>>,
    pub morada: Option<Option<String>>,
    pub cod_postal: Option<Option<String>>,
    pub localidade: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub observacoes: Option<Option<String>>,
}

pub async fn update_customer(
    pool: &SqlitePool,
    id: Uuid,
    upd: CustomerUpdate,
) -> Result<Customer, StorageError> {
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE clientes SET ");
    let mut first = true;
    macro_rules! push_field {
        ($field:ident, $col:literal) => {
            if let Some(v) = upd.$field {
                if !first { q.push(", "); }
                q.push($col).push(" = ");
                q.push_bind(v);
                first = false;
            }
        };
    }
    if let Some(v) = upd.nome {
        if !first { q.push(", "); }
        q.push("nome = ");
        q.push_bind(v);
        first = false;
    }
    push_field!(nif, "nif");
    push_field!(telefone, "telefone");
    push_field!(morada, "morada");
    push_field!(cod_postal, "cod_postal");
    push_field!(localidade, "localidade");
    push_field!(email, "email");
    push_field!(observacoes, "observacoes");

    if first {
        return get_customer(pool, id).await;
    }
    q.push(" WHERE id = ").push_bind(id.to_string());
    q.build().execute(pool).await?;
    get_customer(pool, id).await
}

const DELIVERY_COLS: &str = "id, document_id, cliente_id, morada_snapshot, telefone_snapshot, \
        recebido_em, recebido_via, entregador_id, pronto_em, despachado_em, entregue_em, estado";

fn delivery_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<PedidoDelivery, StorageError> {
    let cliente_id: Option<String> = r.try_get("cliente_id")?;
    let entregador_id: Option<String> = r.try_get("entregador_id")?;
    let estado_s: String = r.try_get("estado")?;
    let estado = DeliveryEstado::parse(&estado_s).ok_or(StorageError::NotFound)?;
    Ok(PedidoDelivery {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        document_id: Uuid::parse_str(r.try_get::<&str, _>("document_id")?)?,
        cliente_id: parse_optional_uuid(cliente_id)?,
        morada_snapshot: r.try_get("morada_snapshot")?,
        telefone_snapshot: r.try_get("telefone_snapshot")?,
        recebido_em: r.try_get::<DateTime<Utc>, _>("recebido_em")?,
        recebido_via: r.try_get("recebido_via")?,
        entregador_id: parse_optional_uuid(entregador_id)?,
        pronto_em: r.try_get::<Option<DateTime<Utc>>, _>("pronto_em")?,
        despachado_em: r.try_get::<Option<DateTime<Utc>>, _>("despachado_em")?,
        entregue_em: r.try_get::<Option<DateTime<Utc>>, _>("entregue_em")?,
        estado,
    })
}

pub async fn create_pedido_delivery(
    pool: &SqlitePool,
    document_id: Uuid,
    cliente_id: Option<Uuid>,
    morada: Option<String>,
    telefone: Option<String>,
    recebido_via: &str,
) -> Result<PedidoDelivery, StorageError> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO pedidos_delivery (id, document_id, cliente_id, morada_snapshot, \
         telefone_snapshot, recebido_em, recebido_via, estado) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'recebido')",
    )
    .bind(id.to_string())
    .bind(document_id.to_string())
    .bind(cliente_id.map(|u| u.to_string()))
    .bind(morada.as_deref())
    .bind(telefone.as_deref())
    .bind(now)
    .bind(recebido_via)
    .execute(pool)
    .await?;
    get_pedido_delivery(pool, id).await
}

pub async fn get_pedido_delivery(
    pool: &SqlitePool,
    id: Uuid,
) -> Result<PedidoDelivery, StorageError> {
    let q = format!("SELECT {DELIVERY_COLS} FROM pedidos_delivery WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    delivery_from_row(&row)
}

pub async fn get_pedido_delivery_by_document(
    pool: &SqlitePool,
    document_id: Uuid,
) -> Result<Option<PedidoDelivery>, StorageError> {
    let q = format!("SELECT {DELIVERY_COLS} FROM pedidos_delivery WHERE document_id = ?1");
    let row = sqlx::query(&q)
        .bind(document_id.to_string())
        .fetch_optional(pool)
        .await?;
    row.as_ref().map(delivery_from_row).transpose()
}

pub async fn list_active_pedidos_delivery(
    pool: &SqlitePool,
) -> Result<Vec<PedidoDelivery>, StorageError> {
    let q = format!(
        "SELECT {DELIVERY_COLS} FROM pedidos_delivery \
         WHERE estado IN ('recebido','em_preparacao','pronto','despachado') \
         ORDER BY recebido_em"
    );
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(delivery_from_row).collect()
}

pub async fn update_delivery_estado(
    pool: &SqlitePool,
    id: Uuid,
    estado: DeliveryEstado,
    entregador_id: Option<Uuid>,
) -> Result<PedidoDelivery, StorageError> {
    let now = Utc::now();
    match estado {
        DeliveryEstado::Pronto => {
            sqlx::query(
                "UPDATE pedidos_delivery SET estado = 'pronto', pronto_em = ?1 WHERE id = ?2",
            )
            .bind(now)
            .bind(id.to_string())
            .execute(pool)
            .await?;
        }
        DeliveryEstado::Despachado => {
            sqlx::query(
                "UPDATE pedidos_delivery SET estado = 'despachado', despachado_em = ?1, \
                 entregador_id = COALESCE(?2, entregador_id) WHERE id = ?3",
            )
            .bind(now)
            .bind(entregador_id.map(|u| u.to_string()))
            .bind(id.to_string())
            .execute(pool)
            .await?;
        }
        DeliveryEstado::Entregue => {
            sqlx::query(
                "UPDATE pedidos_delivery SET estado = 'entregue', entregue_em = ?1 WHERE id = ?2",
            )
            .bind(now)
            .bind(id.to_string())
            .execute(pool)
            .await?;
        }
        DeliveryEstado::EmPreparacao => {
            sqlx::query(
                "UPDATE pedidos_delivery SET estado = 'em_preparacao' WHERE id = ?1",
            )
            .bind(id.to_string())
            .execute(pool)
            .await?;
        }
        DeliveryEstado::Cancelado => {
            sqlx::query(
                "UPDATE pedidos_delivery SET estado = 'cancelado' WHERE id = ?1",
            )
            .bind(id.to_string())
            .execute(pool)
            .await?;
        }
        DeliveryEstado::Recebido => {
            sqlx::query("UPDATE pedidos_delivery SET estado = 'recebido' WHERE id = ?1")
                .bind(id.to_string())
                .execute(pool)
                .await?;
        }
    }
    get_pedido_delivery(pool, id).await
}

pub async fn list_atcuds(pool: &SqlitePool) -> Result<Vec<Atcud>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, document_type, series_prefix, year, atcud, start_date, registered_at, is_active \
         FROM atcud ORDER BY year, document_type, series_prefix, registered_at",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            Ok(Atcud {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                document_type: r.try_get("document_type")?,
                series_prefix: r.try_get("series_prefix")?,
                year: r.try_get::<i64, _>("year")? as i32,
                atcud: r.try_get("atcud")?,
                start_date: r.try_get::<NaiveDate, _>("start_date")?,
                registered_at: r.try_get::<DateTime<Utc>, _>("registered_at")?,
                is_active: r.try_get::<bool, _>("is_active")?,
            })
        })
        .collect()
}
