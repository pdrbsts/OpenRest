use chrono::{DateTime, NaiveDate, Utc};
use domain::{
    Article, Atcud, Document, DocumentDetail, DocumentSeries, Employee, Family, Payment,
    PaymentMethod, Table,
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
    })
}

const DOC_COLS: &str = "id, table_id, employee_id, total, is_closed, created_at, \
        series_id, document_type, document_number, atcud, hash, hash_short, \
        previous_hash, issued_at, qr_payload";

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

pub async fn list_tables(pool: &SqlitePool) -> Result<Vec<Table>, StorageError> {
    let rows = sqlx::query("SELECT id, code, name, is_open FROM tables ORDER BY code")
        .fetch_all(pool)
        .await?;
    rows.into_iter()
        .map(|r| {
            Ok(Table {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                code: r.try_get::<i64, _>("code")? as i32,
                name: r.try_get("name")?,
                is_open: r.try_get::<bool, _>("is_open")?,
            })
        })
        .collect()
}

pub async fn list_employees(pool: &SqlitePool) -> Result<Vec<Employee>, StorageError> {
    let rows = sqlx::query("SELECT id, code, name FROM employees ORDER BY code")
        .fetch_all(pool)
        .await?;
    rows.into_iter()
        .map(|r| {
            Ok(Employee {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                code: r.try_get::<i64, _>("code")? as i32,
                name: r.try_get("name")?,
            })
        })
        .collect()
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
    let row = sqlx::query("SELECT id, code, name, is_open FROM tables WHERE id = ?1")
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    Ok(Table {
        id: Uuid::parse_str(row.try_get::<&str, _>("id")?)?,
        code: row.try_get::<i64, _>("code")? as i32,
        name: row.try_get("name")?,
        is_open: row.try_get::<bool, _>("is_open")?,
    })
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

    let mut tx = pool.begin().await?;
    let doc_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO documents (id, table_id, employee_id, total, is_closed, created_at) \
         VALUES (?1, ?2, ?3, 0, 0, ?4)",
    )
    .bind(doc_id.to_string())
    .bind(table_id.to_string())
    .bind(employee_id.map(|i| i.to_string()))
    .bind(now)
    .execute(&mut *tx)
    .await?;

    sqlx::query("UPDATE tables SET is_open = 1 WHERE id = ?1")
        .bind(table_id.to_string())
        .execute(&mut *tx)
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
    })
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
        sqlx::query("UPDATE tables SET is_open = 0 WHERE id = ?1")
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
