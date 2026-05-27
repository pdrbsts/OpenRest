use chrono::{Datelike, Utc};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::StorageError;

pub async fn seed_if_empty(pool: &SqlitePool) -> Result<bool, StorageError> {
    let row = sqlx::query("SELECT COUNT(*) AS n FROM articles")
        .fetch_one(pool)
        .await?;
    let count: i64 = row.try_get("n")?;
    if count > 0 {
        return Ok(false);
    }

    let now = Utc::now();
    let mut tx = pool.begin().await?;

    // Famílias raiz
    let fam_cafetaria = Uuid::new_v4();
    let fam_comidas = Uuid::new_v4();
    let fam_bebidas = Uuid::new_v4();
    for (id, code, name) in [
        (fam_cafetaria, 10, "Cafetaria"),
        (fam_comidas, 20, "Comidas"),
        (fam_bebidas, 30, "Bebidas"),
    ] {
        sqlx::query(
            "INSERT INTO families (id, parent_id, code, name) VALUES (?1, NULL, ?2, ?3)",
        )
        .bind(id.to_string())
        .bind(code as i64)
        .bind(name)
        .execute(&mut *tx)
        .await?;
    }

    // Sub-famílias (children of parents)
    let sub_cafes = Uuid::new_v4();
    let sub_tostas = Uuid::new_v4();
    let sub_pastel = Uuid::new_v4();
    let sub_aguas = Uuid::new_v4();
    let sub_refri = Uuid::new_v4();
    for (id, parent, code, name) in [
        (sub_cafes, fam_cafetaria, 11, "Cafés"),
        (sub_tostas, fam_comidas, 21, "Tostas e Sandes"),
        (sub_pastel, fam_comidas, 22, "Pastelaria"),
        (sub_aguas, fam_bebidas, 31, "Águas"),
        (sub_refri, fam_bebidas, 32, "Refrigerantes"),
    ] {
        sqlx::query(
            "INSERT INTO families (id, parent_id, code, name) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(id.to_string())
        .bind(parent.to_string())
        .bind(code as i64)
        .bind(name)
        .execute(&mut *tx)
        .await?;
    }

    // Articles — (code, name, price cents, family/sub-family, vat basis points)
    let articles: &[(i64, &str, i64, Uuid, i64)] = &[
        (1, "Café Expresso", 80, sub_cafes, 1300),
        (2, "Meia de Leite", 110, sub_cafes, 1300),
        (3, "Galão", 150, sub_cafes, 1300),
        (4, "Descafeinado", 90, sub_cafes, 1300),
        (10, "Tosta Mista", 300, sub_tostas, 1300),
        (11, "Sandes Fiambre", 250, sub_tostas, 1300),
        (12, "Croissant", 150, sub_pastel, 1300),
        (13, "Pastel de Nata", 130, sub_pastel, 1300),
        (20, "Água 33cl", 120, sub_aguas, 1300),
        (21, "Sumo Laranja", 250, sub_refri, 1300),
        (22, "Coca-Cola 33cl", 230, sub_refri, 2300),
        (23, "Cerveja 33cl", 220, sub_refri, 2300),
    ];
    for (code, name, price, family_id, vat) in articles {
        sqlx::query(
            "INSERT INTO articles (id, family_id, code, name, price, vat_rate, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(family_id.to_string())
        .bind(code)
        .bind(name)
        .bind(price)
        .bind(vat)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    for code in 1..=12 {
        sqlx::query(
            "INSERT INTO tables (id, code, name, is_open) VALUES (?1, ?2, ?3, 0)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(code as i64)
        .bind(format!("Mesa {}", code))
        .execute(&mut *tx)
        .await?;
    }
    sqlx::query("INSERT INTO tables (id, code, name, is_open) VALUES (?1, 99, 'Balcão', 0)")
        .bind(Uuid::new_v4().to_string())
        .execute(&mut *tx)
        .await?;

    for (code, name) in [(1, "Admin"), (2, "Empregado")] {
        sqlx::query("INSERT INTO employees (id, code, name) VALUES (?1, ?2, ?3)")
            .bind(Uuid::new_v4().to_string())
            .bind(code as i64)
            .bind(name)
            .execute(&mut *tx)
            .await?;
    }

    for (code, name) in [(1, "Numerário"), (2, "Multibanco")] {
        sqlx::query("INSERT INTO payment_methods (id, code, name) VALUES (?1, ?2, ?3)")
            .bind(Uuid::new_v4().to_string())
            .bind(code as i64)
            .bind(name)
            .execute(&mut *tx)
            .await?;
    }

    // Série fiscal FS para o ano corrente. Em produção o ATCUD vem da AT;
    // até lá, usamos um placeholder marcado claramente como entidade autónoma.
    let year = now.year();
    let series_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO document_series (id, document_type, prefix, year, next_number, is_active) \
         VALUES (?1, 'FS', 'A', ?2, 1, 1)",
    )
    .bind(series_id.to_string())
    .bind(year as i64)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO atcud (id, document_type, series_prefix, year, atcud, start_date, \
         registered_at, is_active) VALUES (?1, 'FS', 'A', ?2, 'AAOPENREST', ?3, ?4, 1)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(year as i64)
    .bind(now.date_naive())
    .bind(now)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(true)
}
