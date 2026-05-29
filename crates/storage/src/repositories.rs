use chrono::{DateTime, NaiveDate, Utc};
use domain::{
    Anulacao, Article, Atcud, Cancelamento, Customer, DeliveryEstado, Dispositivo, Document,
    DocumentDetail, DocumentSeries, Employee, Entregador, Family, ImpressoraZonaLocal, Local,
    LocalKind, MesaEstado, MesaEstadoKind, NivelAcesso, Payment, PaymentMethod, PedidoDelivery,
    SessaoEmpregado, Table, TipoPreco, Transferencia, Zona, ZonaImpressao,
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
    let sessao_id: Option<String> = row.try_get("sessao_id")?;
    let parent_document_id: Option<String> = row.try_get("parent_document_id").unwrap_or(None);
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
        subtotal_impresso_em: row.try_get::<Option<DateTime<Utc>>, _>("subtotal_impresso_em")?,
        data_dia: row.try_get::<Option<NaiveDate>, _>("data_dia")?,
        sessao_id: parse_optional_uuid(sessao_id)?,
        troco_cents: row.try_get::<i64, _>("troco_cents").unwrap_or(0),
        parent_document_id: parse_optional_uuid(parent_document_id)?,
    })
}

const DOC_COLS: &str = "id, table_id, employee_id, total, is_closed, created_at, \
        series_id, document_type, document_number, atcud, hash, hash_short, \
        previous_hash, issued_at, qr_payload, customer_id, local_id, \
        observacoes_pedido, observacoes_factura, observacoes_cliente, \
        observacoes_morada, delivery_morada, delivery_telefone, subtotal_impresso_em, \
        data_dia, sessao_id, troco_cents, parent_document_id";

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

const ARTICLE_COLS: &str = "id, family_id, code, name, pvp1, pvp2, pvp3, pvp4, pvp5, vat_rate, \
        tipo_artigo, zona_impressao_id, created_at, updated_at";

fn article_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Article, StorageError> {
    let family_id: Option<String> = r.try_get("family_id")?;
    let zona: Option<String> = r.try_get("zona_impressao_id")?;
    Ok(Article {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        family_id: parse_optional_uuid(family_id)?,
        code: r.try_get::<i64, _>("code")? as i32,
        name: r.try_get("name")?,
        pvp1: r.try_get::<i64, _>("pvp1")?,
        pvp2: r.try_get::<Option<i64>, _>("pvp2")?,
        pvp3: r.try_get::<Option<i64>, _>("pvp3")?,
        pvp4: r.try_get::<Option<i64>, _>("pvp4")?,
        pvp5: r.try_get::<Option<i64>, _>("pvp5")?,
        vat_rate: r.try_get::<i64, _>("vat_rate")? as i32,
        tipo_artigo: r.try_get("tipo_artigo")?,
        zona_impressao_id: parse_optional_uuid(zona)?,
        created_at: r.try_get::<DateTime<Utc>, _>("created_at")?,
        updated_at: r.try_get::<DateTime<Utc>, _>("updated_at")?,
    })
}

pub async fn list_articles(pool: &SqlitePool) -> Result<Vec<Article>, StorageError> {
    let q = format!("SELECT {ARTICLE_COLS} FROM articles ORDER BY code");
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(article_from_row).collect()
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

fn employee_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Employee, StorageError> {
    let nivel: Option<String> = r.try_get("nivel_acesso_id")?;
    Ok(Employee {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        code: r.try_get::<i64, _>("code")? as i32,
        name: r.try_get("name")?,
        perc_consumo: r.try_get::<i64, _>("perc_consumo")? as i32,
        base_consumo: r.try_get::<i64, _>("base_consumo")?,
        nivel_acesso_id: parse_optional_uuid(nivel)?,
    })
}

pub async fn list_employees(pool: &SqlitePool) -> Result<Vec<Employee>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, code, name, perc_consumo, base_consumo, nivel_acesso_id \
         FROM employees ORDER BY code",
    )
    .fetch_all(pool)
    .await?;
    rows.iter().map(employee_from_row).collect()
}

pub async fn get_employee(pool: &SqlitePool, id: Uuid) -> Result<Employee, StorageError> {
    let r = sqlx::query(
        "SELECT id, code, name, perc_consumo, base_consumo, nivel_acesso_id \
         FROM employees WHERE id = ?1",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?
    .ok_or(StorageError::NotFound)?;
    employee_from_row(&r)
}

const NIVEL_COLS: &str = "id, codigo, designacao, cancela_pedidos, anula_pedidos, \
        anula_pedidos_com_conta_impressa, transfere_pedidos, \
        transfere_pedidos_com_conta_impressa, anulado_em";

fn nivel_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<NivelAcesso, StorageError> {
    Ok(NivelAcesso {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        codigo: r.try_get::<i64, _>("codigo")? as i32,
        designacao: r.try_get("designacao")?,
        cancela_pedidos: r.try_get::<bool, _>("cancela_pedidos")?,
        anula_pedidos: r.try_get::<bool, _>("anula_pedidos")?,
        anula_pedidos_com_conta_impressa: r
            .try_get::<bool, _>("anula_pedidos_com_conta_impressa")?,
        transfere_pedidos: r.try_get::<bool, _>("transfere_pedidos")?,
        transfere_pedidos_com_conta_impressa: r
            .try_get::<bool, _>("transfere_pedidos_com_conta_impressa")?,
        anulado_em: r.try_get::<Option<DateTime<Utc>>, _>("anulado_em")?,
    })
}

pub async fn list_niveis_acesso(pool: &SqlitePool) -> Result<Vec<NivelAcesso>, StorageError> {
    let q = format!(
        "SELECT {NIVEL_COLS} FROM niveis_acesso WHERE anulado_em IS NULL ORDER BY codigo"
    );
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(nivel_from_row).collect()
}

pub async fn get_nivel_acesso(pool: &SqlitePool, id: Uuid) -> Result<NivelAcesso, StorageError> {
    let q = format!("SELECT {NIVEL_COLS} FROM niveis_acesso WHERE id = ?1");
    let r = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    nivel_from_row(&r)
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
    let q = format!("SELECT {ARTICLE_COLS} FROM articles WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    article_from_row(&row)
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

const DETAIL_COLS: &str = "id, document_id, article_id, qty, qty_milli, unit_price, total, \
        pedida_em, anulada, anulada_com_desperdicio, anulada_em, anulada_por, anulada_motivo, \
        descricao";

fn detail_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<DocumentDetail, StorageError> {
    let anulada_por: Option<String> = r.try_get("anulada_por")?;
    let qty: i32 = r.try_get::<i64, _>("qty")? as i32;
    // Linhas pré-migration têm qty_milli = NULL — derivamos a partir de qty.
    let qty_milli: i64 = r
        .try_get::<Option<i64>, _>("qty_milli")?
        .unwrap_or((qty as i64) * 1000);
    Ok(DocumentDetail {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        document_id: Uuid::parse_str(r.try_get::<&str, _>("document_id")?)?,
        article_id: Uuid::parse_str(r.try_get::<&str, _>("article_id")?)?,
        qty,
        qty_milli,
        unit_price: r.try_get::<i64, _>("unit_price")?,
        total: r.try_get::<i64, _>("total")?,
        pedida_em: r.try_get::<Option<DateTime<Utc>>, _>("pedida_em")?,
        anulada: r.try_get::<bool, _>("anulada")?,
        anulada_com_desperdicio: r.try_get::<bool, _>("anulada_com_desperdicio")?,
        anulada_em: r.try_get::<Option<DateTime<Utc>>, _>("anulada_em")?,
        anulada_por: parse_optional_uuid(anulada_por)?,
        anulada_motivo: r.try_get("anulada_motivo")?,
        descricao: r.try_get::<Option<String>, _>("descricao")?,
    })
}

pub async fn list_document_details(
    pool: &SqlitePool,
    document_id: Uuid,
) -> Result<Vec<DocumentDetail>, StorageError> {
    let q = format!(
        "SELECT {DETAIL_COLS} FROM document_details WHERE document_id = ?1 ORDER BY rowid"
    );
    let rows = sqlx::query(&q)
        .bind(document_id.to_string())
        .fetch_all(pool)
        .await?;
    rows.iter().map(detail_from_row).collect()
}

pub async fn get_document_detail(
    pool: &SqlitePool,
    line_id: Uuid,
) -> Result<DocumentDetail, StorageError> {
    let q = format!("SELECT {DETAIL_COLS} FROM document_details WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(line_id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    detail_from_row(&row)
}

/// Cancela (apaga fisicamente) uma linha. Só permitido enquanto a linha não foi
/// pedida (pedida_em IS NULL). Decrementa total do documento e subtotal_actual
/// da mesa de forma transaccional. Se `record_audit` for true, regista em
/// `cancelamentos` (spec 03 §11 — registo opcional, configurável).
pub async fn cancel_document_line(
    pool: &SqlitePool,
    document_id: Uuid,
    line_id: Uuid,
    record_audit: bool,
    motivo: Option<String>,
    empregado_id: Option<Uuid>,
) -> Result<Option<Cancelamento>, StorageError> {
    let line = get_document_detail(pool, line_id).await?;
    if line.document_id != document_id {
        return Err(StorageError::NotFound);
    }
    if line.pedida_em.is_some() {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "linha já pedida — usar anular".into(),
        )));
    }
    if line.anulada {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "linha já anulada".into(),
        )));
    }

    let now = Utc::now();
    let cancelamento_id = Uuid::new_v4();
    let mut tx = pool.begin().await?;

    if record_audit {
        sqlx::query(
            "INSERT INTO cancelamentos (id, document_id, article_id, qty, unit_price, total, \
             motivo, empregado_id, cancelada_em) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        )
        .bind(cancelamento_id.to_string())
        .bind(document_id.to_string())
        .bind(line.article_id.to_string())
        .bind(line.qty as i64)
        .bind(line.unit_price)
        .bind(line.total)
        .bind(motivo.as_deref())
        .bind(empregado_id.map(|u| u.to_string()))
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query("DELETE FROM document_details WHERE id = ?1")
        .bind(line_id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query("UPDATE documents SET total = total - ?1 WHERE id = ?2")
        .bind(line.total)
        .bind(document_id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        "UPDATE mesa_estado SET subtotal_actual = subtotal_actual - ?1 \
         WHERE mesa_id = (SELECT table_id FROM documents WHERE id = ?2)",
    )
    .bind(line.total)
    .bind(document_id.to_string())
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    if record_audit {
        Ok(Some(Cancelamento {
            id: cancelamento_id,
            document_id,
            article_id: line.article_id,
            qty: line.qty,
            unit_price: line.unit_price,
            total: line.total,
            motivo,
            empregado_id,
            cancelada_em: now,
        }))
    } else {
        Ok(None)
    }
}

/// Anula (marca inline) uma linha já pedida. Idempotente: anular duas vezes erra.
/// Decrementa total/subtotal mas mantém a linha para histórico.
pub async fn anular_document_line(
    pool: &SqlitePool,
    document_id: Uuid,
    line_id: Uuid,
    com_desperdicio: bool,
    motivo: Option<String>,
    empregado_id: Option<Uuid>,
) -> Result<DocumentDetail, StorageError> {
    let line = get_document_detail(pool, line_id).await?;
    if line.document_id != document_id {
        return Err(StorageError::NotFound);
    }
    if line.pedida_em.is_none() {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "linha ainda não pedida — usar cancelar".into(),
        )));
    }
    if line.anulada {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "linha já anulada".into(),
        )));
    }

    let now = Utc::now();
    let anulacao_id = Uuid::new_v4();
    let mut tx = pool.begin().await?;

    // Auditoria sempre registada (spec — Anulação é sempre registada).
    sqlx::query(
        "INSERT INTO anulacoes (id, document_id, document_detail_id, article_id, qty, \
         unit_price, total, com_desperdicio, motivo, empregado_id, anulada_em) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
    )
    .bind(anulacao_id.to_string())
    .bind(document_id.to_string())
    .bind(line_id.to_string())
    .bind(line.article_id.to_string())
    .bind(line.qty as i64)
    .bind(line.unit_price)
    .bind(line.total)
    .bind(com_desperdicio as i32)
    .bind(motivo.as_deref())
    .bind(empregado_id.map(|u| u.to_string()))
    .bind(now)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE document_details SET anulada = 1, anulada_com_desperdicio = ?1, \
         anulada_em = ?2, anulada_por = ?3, anulada_motivo = ?4 WHERE id = ?5",
    )
    .bind(com_desperdicio as i32)
    .bind(now)
    .bind(empregado_id.map(|u| u.to_string()))
    .bind(motivo.as_deref())
    .bind(line_id.to_string())
    .execute(&mut *tx)
    .await?;
    sqlx::query("UPDATE documents SET total = total - ?1 WHERE id = ?2")
        .bind(line.total)
        .bind(document_id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        "UPDATE mesa_estado SET subtotal_actual = subtotal_actual - ?1 \
         WHERE mesa_id = (SELECT table_id FROM documents WHERE id = ?2)",
    )
    .bind(line.total)
    .bind(document_id.to_string())
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    get_document_detail(pool, line_id).await
}

/// Transfere linhas (ou todas) de `from_document` para uma nova mesa destino.
/// Spec §9: linhas anuladas não viajam; cria document destino se a mesa estiver
/// livre; recalcula totais e mesa_estado em ambos os lados; regista auditoria.
pub async fn transfer_document_lines(
    pool: &SqlitePool,
    from_document_id: Uuid,
    target_table_id: Uuid,
    line_ids: Option<&[Uuid]>,
    employee_id: Option<Uuid>,
    business_date: NaiveDate,
) -> Result<(Document, Vec<Transferencia>), StorageError> {
    let from_doc = get_document(pool, from_document_id).await?;
    if from_doc.is_closed {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "documento origem já fechado".into(),
        )));
    }
    let from_table_id = from_doc.table_id.ok_or_else(|| {
        StorageError::Database(sqlx::Error::Protocol(
            "documento origem não está associado a uma mesa".into(),
        ))
    })?;
    if from_table_id == target_table_id {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "mesa destino igual à origem".into(),
        )));
    }
    let target_table = get_table(pool, target_table_id).await?;
    // Bloquear consumo próprio na origem ou destino (spec §163-167).
    if let Some(local_id) = from_doc.local_id {
        let local = get_local(pool, local_id).await?;
        if local.tipo == LocalKind::ConsumoProprio {
            return Err(StorageError::Database(sqlx::Error::Protocol(
                "consumo próprio não transfere".into(),
            )));
        }
    }
    if let Some(local_id) = target_table.local_id {
        let local = get_local(pool, local_id).await?;
        if local.tipo == LocalKind::ConsumoProprio {
            return Err(StorageError::Database(sqlx::Error::Protocol(
                "destino é consumo próprio".into(),
            )));
        }
    }

    // Lista activa de linhas (não anuladas) na origem.
    let q = format!(
        "SELECT {DETAIL_COLS} FROM document_details \
         WHERE document_id = ?1 AND anulada = 0"
    );
    let rows = sqlx::query(&q)
        .bind(from_document_id.to_string())
        .fetch_all(pool)
        .await?;
    let all: Vec<DocumentDetail> = rows.iter().map(detail_from_row).collect::<Result<_, _>>()?;
    let selected: Vec<DocumentDetail> = match line_ids {
        Some(ids) if !ids.is_empty() => {
            let set: std::collections::HashSet<Uuid> = ids.iter().copied().collect();
            all.into_iter().filter(|l| set.contains(&l.id)).collect()
        }
        _ => all,
    };
    if selected.is_empty() {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "sem linhas para transferir".into(),
        )));
    }

    // Garante documento destino (cria se a mesa estiver livre).
    let to_doc = match get_open_document_for_table(pool, target_table_id).await? {
        Some(d) => d,
        None => open_table(pool, target_table_id, employee_id, business_date).await?,
    };
    let to_document_id = to_doc.id;

    let now = Utc::now();
    let mut tx = pool.begin().await?;
    let mut transferencias = Vec::with_capacity(selected.len());

    let total_moved: i64 = selected.iter().map(|l| l.total).sum();

    for line in &selected {
        sqlx::query(
            "UPDATE document_details SET document_id = ?1 WHERE id = ?2 AND document_id = ?3",
        )
        .bind(to_document_id.to_string())
        .bind(line.id.to_string())
        .bind(from_document_id.to_string())
        .execute(&mut *tx)
        .await?;

        let transfer_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO transferencias (id, from_document_id, to_document_id, line_id, \
             article_id, qty, employee_id, transferida_em) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind(transfer_id.to_string())
        .bind(from_document_id.to_string())
        .bind(to_document_id.to_string())
        .bind(line.id.to_string())
        .bind(line.article_id.to_string())
        .bind(line.qty as i64)
        .bind(employee_id.map(|u| u.to_string()))
        .bind(now)
        .execute(&mut *tx)
        .await?;

        transferencias.push(Transferencia {
            id: transfer_id,
            from_document_id,
            to_document_id,
            line_id: line.id,
            article_id: line.article_id,
            qty: line.qty as i64,
            employee_id,
            transferida_em: now,
        });
    }

    sqlx::query("UPDATE documents SET total = total - ?1 WHERE id = ?2")
        .bind(total_moved)
        .bind(from_document_id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query("UPDATE documents SET total = total + ?1 WHERE id = ?2")
        .bind(total_moved)
        .bind(to_document_id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        "UPDATE mesa_estado SET subtotal_actual = subtotal_actual - ?1 WHERE mesa_id = ?2",
    )
    .bind(total_moved)
    .bind(from_table_id.to_string())
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "UPDATE mesa_estado SET subtotal_actual = subtotal_actual + ?1 WHERE mesa_id = ?2",
    )
    .bind(total_moved)
    .bind(target_table_id.to_string())
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    let to_doc = get_document(pool, to_document_id).await?;
    Ok((to_doc, transferencias))
}

pub async fn list_transferencias(pool: &SqlitePool) -> Result<Vec<Transferencia>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, from_document_id, to_document_id, line_id, article_id, qty, employee_id, \
         transferida_em FROM transferencias ORDER BY transferida_em DESC",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            let emp: Option<String> = r.try_get("employee_id")?;
            Ok(Transferencia {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                from_document_id: Uuid::parse_str(r.try_get::<&str, _>("from_document_id")?)?,
                to_document_id: Uuid::parse_str(r.try_get::<&str, _>("to_document_id")?)?,
                line_id: Uuid::parse_str(r.try_get::<&str, _>("line_id")?)?,
                article_id: Uuid::parse_str(r.try_get::<&str, _>("article_id")?)?,
                qty: r.try_get::<i64, _>("qty")?,
                employee_id: parse_optional_uuid(emp)?,
                transferida_em: r.try_get::<DateTime<Utc>, _>("transferida_em")?,
            })
        })
        .collect()
}

pub async fn list_anulacoes(pool: &SqlitePool) -> Result<Vec<Anulacao>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, document_id, document_detail_id, article_id, qty, unit_price, total, \
         com_desperdicio, motivo, empregado_id, anulada_em \
         FROM anulacoes ORDER BY anulada_em DESC",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            let emp: Option<String> = r.try_get("empregado_id")?;
            Ok(Anulacao {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                document_id: Uuid::parse_str(r.try_get::<&str, _>("document_id")?)?,
                document_detail_id: Uuid::parse_str(r.try_get::<&str, _>("document_detail_id")?)?,
                article_id: Uuid::parse_str(r.try_get::<&str, _>("article_id")?)?,
                qty: r.try_get::<i64, _>("qty")? as i32,
                unit_price: r.try_get::<i64, _>("unit_price")?,
                total: r.try_get::<i64, _>("total")?,
                com_desperdicio: r.try_get::<bool, _>("com_desperdicio")?,
                motivo: r.try_get("motivo")?,
                empregado_id: parse_optional_uuid(emp)?,
                anulada_em: r.try_get::<DateTime<Utc>, _>("anulada_em")?,
            })
        })
        .collect()
}

pub async fn list_cancelamentos(pool: &SqlitePool) -> Result<Vec<Cancelamento>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, document_id, article_id, qty, unit_price, total, motivo, empregado_id, \
         cancelada_em FROM cancelamentos ORDER BY cancelada_em DESC",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            let emp: Option<String> = r.try_get("empregado_id")?;
            Ok(Cancelamento {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                document_id: Uuid::parse_str(r.try_get::<&str, _>("document_id")?)?,
                article_id: Uuid::parse_str(r.try_get::<&str, _>("article_id")?)?,
                qty: r.try_get::<i64, _>("qty")? as i32,
                unit_price: r.try_get::<i64, _>("unit_price")?,
                total: r.try_get::<i64, _>("total")?,
                motivo: r.try_get("motivo")?,
                empregado_id: parse_optional_uuid(emp)?,
                cancelada_em: r.try_get::<DateTime<Utc>, _>("cancelada_em")?,
            })
        })
        .collect()
}

pub async fn mark_lines_pedidas(
    pool: &SqlitePool,
    document_id: Uuid,
    line_ids: &[Uuid],
) -> Result<(), StorageError> {
    if line_ids.is_empty() {
        return Ok(());
    }
    let now = Utc::now();
    let mut tx = pool.begin().await?;
    for id in line_ids {
        sqlx::query(
            "UPDATE document_details SET pedida_em = ?1 WHERE id = ?2 AND document_id = ?3 \
             AND pedida_em IS NULL",
        )
        .bind(now)
        .bind(id.to_string())
        .bind(document_id.to_string())
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

// --- Sessões de empregado (spec §7.4) ----------------------------------------

const SESSAO_COLS: &str = "id, empregado_id, data_dia, com_bolsa, fundo_bolsa, \
        observacao_abertura, observacao_fecho, aberta_em, aberta_por, fechada_em, \
        fechada_por";

fn sessao_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<SessaoEmpregado, StorageError> {
    let aberta_por: Option<String> = r.try_get("aberta_por")?;
    let fechada_por: Option<String> = r.try_get("fechada_por")?;
    Ok(SessaoEmpregado {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        empregado_id: Uuid::parse_str(r.try_get::<&str, _>("empregado_id")?)?,
        data_dia: r.try_get::<NaiveDate, _>("data_dia")?,
        com_bolsa: r.try_get::<bool, _>("com_bolsa")?,
        fundo_bolsa: r.try_get::<i64, _>("fundo_bolsa")?,
        observacao_abertura: r.try_get("observacao_abertura")?,
        observacao_fecho: r.try_get("observacao_fecho")?,
        aberta_em: r.try_get::<DateTime<Utc>, _>("aberta_em")?,
        aberta_por: parse_optional_uuid(aberta_por)?,
        fechada_em: r.try_get::<Option<DateTime<Utc>>, _>("fechada_em")?,
        fechada_por: parse_optional_uuid(fechada_por)?,
    })
}

pub async fn get_open_sessao_for_employee(
    pool: &SqlitePool,
    empregado_id: Uuid,
) -> Result<Option<SessaoEmpregado>, StorageError> {
    let q = format!(
        "SELECT {SESSAO_COLS} FROM sessoes_empregado \
         WHERE empregado_id = ?1 AND fechada_em IS NULL"
    );
    let row = sqlx::query(&q)
        .bind(empregado_id.to_string())
        .fetch_optional(pool)
        .await?;
    row.as_ref().map(sessao_from_row).transpose()
}

pub async fn get_sessao(pool: &SqlitePool, id: Uuid) -> Result<SessaoEmpregado, StorageError> {
    let q = format!("SELECT {SESSAO_COLS} FROM sessoes_empregado WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    sessao_from_row(&row)
}

#[derive(Default)]
pub struct NewSessao {
    pub empregado_id: Uuid,
    pub data_dia: NaiveDate,
    pub com_bolsa: bool,
    pub fundo_bolsa: i64,
    pub observacao_abertura: Option<String>,
    pub aberta_por: Option<Uuid>,
}

/// Spec §30: erro se já existe uma sessão aberta para este empregado. O índice
/// único parcial garante consistência mesmo em race conditions.
pub async fn open_sessao(
    pool: &SqlitePool,
    input: NewSessao,
) -> Result<SessaoEmpregado, StorageError> {
    if let Some(existing) = get_open_sessao_for_employee(pool, input.empregado_id).await? {
        return Err(StorageError::Database(sqlx::Error::Protocol(format!(
            "empregado já tem sessão aberta ({})",
            existing.id
        ))));
    }
    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO sessoes_empregado (id, empregado_id, data_dia, com_bolsa, fundo_bolsa, \
         observacao_abertura, aberta_em, aberta_por) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )
    .bind(id.to_string())
    .bind(input.empregado_id.to_string())
    .bind(input.data_dia)
    .bind(input.com_bolsa as i32)
    .bind(input.fundo_bolsa)
    .bind(input.observacao_abertura.as_deref())
    .bind(now)
    .bind(input.aberta_por.map(|u| u.to_string()))
    .execute(pool)
    .await?;
    get_sessao(pool, id).await
}

/// Spec §29: só fecha se não houver mesas abertas pelo empregado em locais que
/// não permitem `permite_mesas_abertas_fim_do_dia`. Os documentos sem mesa
/// (take-away/delivery) também contam — devem estar fechados.
pub async fn close_sessao(
    pool: &SqlitePool,
    id: Uuid,
    observacao: Option<String>,
    fechada_por: Option<Uuid>,
) -> Result<SessaoEmpregado, StorageError> {
    let sessao = get_sessao(pool, id).await?;
    if sessao.fechada_em.is_some() {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "sessão já está fechada".into(),
        )));
    }

    // Documentos abertos do empregado em locais que NÃO permitem mesas abertas.
    let blockers: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM documents d \
         LEFT JOIN locais l ON l.id = d.local_id \
         WHERE d.employee_id = ?1 AND d.is_closed = 0 \
           AND COALESCE(l.permite_mesas_abertas_fim_do_dia, 0) = 0",
    )
    .bind(sessao.empregado_id.to_string())
    .fetch_one(pool)
    .await?;
    if blockers > 0 {
        return Err(StorageError::Database(sqlx::Error::Protocol(format!(
            "empregado tem {blockers} documento(s) por fechar em locais sem 'mesas abertas fim do dia'"
        ))));
    }

    let now = Utc::now();
    sqlx::query(
        "UPDATE sessoes_empregado SET fechada_em = ?1, fechada_por = ?2, \
         observacao_fecho = ?3 WHERE id = ?4",
    )
    .bind(now)
    .bind(fechada_por.map(|u| u.to_string()))
    .bind(observacao.as_deref())
    .bind(id.to_string())
    .execute(pool)
    .await?;
    get_sessao(pool, id).await
}

pub async fn list_sessoes(
    pool: &SqlitePool,
    apenas_abertas: bool,
) -> Result<Vec<SessaoEmpregado>, StorageError> {
    let q = if apenas_abertas {
        format!(
            "SELECT {SESSAO_COLS} FROM sessoes_empregado \
             WHERE fechada_em IS NULL ORDER BY aberta_em DESC"
        )
    } else {
        format!(
            "SELECT {SESSAO_COLS} FROM sessoes_empregado \
             ORDER BY aberta_em DESC LIMIT 100"
        )
    };
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(sessao_from_row).collect()
}

// --- Documentos --------------------------------------------------------------

pub async fn open_table(
    pool: &SqlitePool,
    table_id: Uuid,
    employee_id: Option<Uuid>,
    business_date: NaiveDate,
) -> Result<Document, StorageError> {
    if let Some(existing) = get_open_document_for_table(pool, table_id).await? {
        return Ok(existing);
    }

    let sessao_id = match employee_id {
        Some(eid) => get_open_sessao_for_employee(pool, eid).await?.map(|s| s.id),
        None => None,
    };

    let table = get_table(pool, table_id).await?;
    let mut tx = pool.begin().await?;
    let doc_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO documents (id, table_id, employee_id, total, is_closed, created_at, \
         local_id, data_dia, sessao_id) VALUES (?1, ?2, ?3, 0, 0, ?4, ?5, ?6, ?7)",
    )
    .bind(doc_id.to_string())
    .bind(table_id.to_string())
    .bind(employee_id.map(|i| i.to_string()))
    .bind(now)
    .bind(table.local_id.map(|u| u.to_string()))
    .bind(business_date)
    .bind(sessao_id.map(|u| u.to_string()))
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
        subtotal_impresso_em: None,
        data_dia: Some(business_date),
        sessao_id,
        troco_cents: 0,
        parent_document_id: None,
    })
}

/// Cria um documento solto, ligado apenas a um local (sem mesa física).
/// Usado pelos modos take_away (balcão) e delivery (encomenda nova).
pub async fn start_document_for_local(
    pool: &SqlitePool,
    local_id: Uuid,
    employee_id: Option<Uuid>,
    business_date: NaiveDate,
) -> Result<Document, StorageError> {
    let sessao_id = match employee_id {
        Some(eid) => get_open_sessao_for_employee(pool, eid).await?.map(|s| s.id),
        None => None,
    };
    let doc_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO documents (id, table_id, employee_id, total, is_closed, created_at, \
         local_id, data_dia, sessao_id) VALUES (?1, NULL, ?2, 0, 0, ?3, ?4, ?5, ?6)",
    )
    .bind(doc_id.to_string())
    .bind(employee_id.map(|i| i.to_string()))
    .bind(now)
    .bind(local_id.to_string())
    .bind(business_date)
    .bind(sessao_id.map(|u| u.to_string()))
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
    let qty_milli = (qty as i64) * 1000;

    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT INTO document_details (id, document_id, article_id, qty, qty_milli, unit_price, total) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )
    .bind(detail_id.to_string())
    .bind(document_id.to_string())
    .bind(article_id.to_string())
    .bind(qty as i64)
    .bind(qty_milli)
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
        qty_milli,
        unit_price,
        total,
        pedida_em: None,
        anulada: false,
        anulada_com_desperdicio: false,
        anulada_em: None,
        anulada_por: None,
        anulada_motivo: None,
        descricao: None,
    })
}

pub async fn list_document_payments(
    pool: &SqlitePool,
    document_id: Uuid,
) -> Result<Vec<Payment>, StorageError> {
    let rows = sqlx::query(
        "SELECT id, document_id, payment_method_id, amount, descricao, created_at \
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
                descricao: r.try_get::<Option<String>, _>("descricao")?,
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
    let mut tx = pool.begin().await?;
    let payment = record_payment_tx(&mut tx, document_id, payment_method_id, amount, None).await?;
    tx.commit().await?;
    Ok(payment)
}

/// Variante transaccional de `record_payment`: usada quando o fecho fiscal
/// precisa de inserir N rodapés de pagamento na mesma transacção. Aceita uma
/// descrição opcional (campo da janela Avançada).
pub async fn record_payment_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    document_id: Uuid,
    payment_method_id: Uuid,
    amount: i64,
    descricao: Option<String>,
) -> Result<Payment, StorageError> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO payments (id, document_id, payment_method_id, amount, descricao, created_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )
    .bind(id.to_string())
    .bind(document_id.to_string())
    .bind(payment_method_id.to_string())
    .bind(amount)
    .bind(descricao.as_deref())
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(Payment {
        id,
        document_id,
        payment_method_id,
        amount,
        descricao,
        created_at: now,
    })
}

#[derive(Debug, Clone)]
pub struct PaymentInput {
    pub payment_method_id: Uuid,
    pub amount: i64,
    pub descricao: Option<String>,
}

/// Insere N rodapés de pagamento atomicamente e actualiza `documents.troco_cents`
/// com o eventual excedente (soma_pagamentos − total). Não valida soma >= total
/// — a chamada (camada API) é responsável por essa regra antes de invocar.
pub async fn record_payments_bulk_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    document_id: Uuid,
    document_total: i64,
    payments: &[PaymentInput],
) -> Result<(Vec<Payment>, i64), StorageError> {
    let mut out = Vec::with_capacity(payments.len());
    let mut sum: i64 = 0;
    for p in payments {
        sum += p.amount;
        out.push(
            record_payment_tx(tx, document_id, p.payment_method_id, p.amount, p.descricao.clone())
                .await?,
        );
    }
    let troco = (sum - document_total).max(0);
    sqlx::query("UPDATE documents SET troco_cents = ?1 WHERE id = ?2")
        .bind(troco)
        .bind(document_id.to_string())
        .execute(&mut **tx)
        .await?;
    Ok((out, troco))
}

/// Cria um Document filho de um pagamento parcial / divisão de conta. O filho
/// herda local/empregado/sessão do pai mas **não recebe `table_id`** — isso
/// garante que o fecho fiscal do filho não liberta a mesa, e o pai mantém a
/// posse da mesa enquanto restarem linhas por liquidar.
async fn create_child_document_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    parent: &Document,
) -> Result<Document, StorageError> {
    let child_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO documents (id, table_id, employee_id, total, is_closed, created_at, \
         local_id, data_dia, sessao_id, parent_document_id) \
         VALUES (?1, NULL, ?2, 0, 0, ?3, ?4, ?5, ?6, ?7)",
    )
    .bind(child_id.to_string())
    .bind(parent.employee_id.map(|i| i.to_string()))
    .bind(now)
    .bind(parent.local_id.map(|i| i.to_string()))
    .bind(parent.data_dia)
    .bind(parent.sessao_id.map(|i| i.to_string()))
    .bind(parent.id.to_string())
    .execute(&mut **tx)
    .await?;
    Ok(Document {
        id: child_id,
        table_id: None,
        employee_id: parent.employee_id,
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
        local_id: parent.local_id,
        observacoes_pedido: None,
        observacoes_factura: None,
        observacoes_cliente: None,
        observacoes_morada: None,
        delivery_morada: None,
        delivery_telefone: None,
        subtotal_impresso_em: None,
        data_dia: parent.data_dia,
        sessao_id: parent.sessao_id,
        troco_cents: 0,
        parent_document_id: Some(parent.id),
    })
}

/// Move um conjunto de linhas pedidas (não anuladas) do pai para um novo
/// documento-filho. Recalcula totais do pai e do filho na mesma transacção e
/// decrementa `mesa_estado.subtotal_actual` na medida do total movido (o
/// crédito permanece "na mesa" apenas enquanto está no pai).
///
/// Restrições:
/// * Linhas devem pertencer a `parent_id`.
/// * Apenas linhas com `pedida_em IS NOT NULL` e `anulada = 0` são elegíveis
///   (linhas em construção devem ser canceladas/pedidas antes).
/// * Pelo menos uma linha tem de ser movida.
pub async fn move_lines_to_new_document(
    pool: &SqlitePool,
    parent_id: Uuid,
    line_ids: &[Uuid],
) -> Result<Document, StorageError> {
    if line_ids.is_empty() {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "no lines selected for partial close".into(),
        )));
    }
    let parent = get_document(pool, parent_id).await?;
    if parent.is_closed {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "parent document already closed".into(),
        )));
    }

    let mut tx = pool.begin().await?;
    let child = create_child_document_tx(&mut tx, &parent).await?;

    let mut moved_total: i64 = 0;
    for line_id in line_ids {
        let row = sqlx::query(
            "SELECT document_id, total, pedida_em, anulada FROM document_details WHERE id = ?1",
        )
        .bind(line_id.to_string())
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(StorageError::NotFound)?;
        let owner: String = row.try_get("document_id")?;
        if Uuid::parse_str(&owner)? != parent_id {
            return Err(StorageError::Database(sqlx::Error::Protocol(
                "line does not belong to parent".into(),
            )));
        }
        let pedida_em: Option<DateTime<Utc>> = row.try_get("pedida_em")?;
        let anulada: bool = row.try_get("anulada")?;
        if pedida_em.is_none() {
            return Err(StorageError::Database(sqlx::Error::Protocol(
                "line not yet ordered (pedida_em NULL)".into(),
            )));
        }
        if anulada {
            return Err(StorageError::Database(sqlx::Error::Protocol(
                "anulada lines cannot move".into(),
            )));
        }
        let line_total: i64 = row.try_get("total")?;
        sqlx::query("UPDATE document_details SET document_id = ?1 WHERE id = ?2")
            .bind(child.id.to_string())
            .bind(line_id.to_string())
            .execute(&mut *tx)
            .await?;
        moved_total += line_total;
    }

    sqlx::query("UPDATE documents SET total = total - ?1 WHERE id = ?2")
        .bind(moved_total)
        .bind(parent_id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query("UPDATE documents SET total = total + ?1 WHERE id = ?2")
        .bind(moved_total)
        .bind(child.id.to_string())
        .execute(&mut *tx)
        .await?;
    if let Some(table_id) = parent.table_id {
        sqlx::query(
            "UPDATE mesa_estado SET subtotal_actual = subtotal_actual - ?1 WHERE mesa_id = ?2",
        )
        .bind(moved_total)
        .bind(table_id.to_string())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    get_document(pool, child.id).await
}

#[derive(Debug, Clone)]
pub struct SplitAssignment {
    /// Linhas atribuídas a esta conta-filho. Pode ser vazio (filho começa
    /// vazio — útil quando o operador vai depois mover linhas pela UI).
    pub line_ids: Vec<Uuid>,
}

/// Divide um documento em N filhos. O `assignments.len()` define N. O pai
/// fica marcado `is_closed=true` sem dados fiscais (registo operacional);
/// cada filho corre na cadeia fiscal de forma independente.
///
/// Caller-side concerns:
/// * `assignments[i].line_ids` é a lista exacta de linhas para o filho i.
/// * Linhas omitidas das atribuições ficam no pai (que se mantém aberto sem
///   `is_closed=true` se sobrar pelo menos uma).
/// * Se TODAS as linhas elegíveis forem atribuídas, o pai fica vazio e é
///   marcado split-closed; senão fica aberto com as linhas remanescentes.
pub async fn split_document(
    pool: &SqlitePool,
    parent_id: Uuid,
    assignments: &[SplitAssignment],
) -> Result<Vec<Document>, StorageError> {
    if assignments.is_empty() {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "split requires at least one account".into(),
        )));
    }
    let parent = get_document(pool, parent_id).await?;
    if parent.is_closed {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "parent document already closed".into(),
        )));
    }

    // Linhas elegíveis do pai (pedidas e não anuladas). Restantes ficam no pai.
    let all_lines = list_document_details(pool, parent_id).await?;
    let eligible: std::collections::HashSet<Uuid> = all_lines
        .iter()
        .filter(|l| l.pedida_em.is_some() && !l.anulada)
        .map(|l| l.id)
        .collect();

    let mut tx = pool.begin().await?;
    let mut created = Vec::with_capacity(assignments.len());
    let mut total_moved: i64 = 0;

    for assignment in assignments {
        let child = create_child_document_tx(&mut tx, &parent).await?;
        let mut child_total: i64 = 0;
        for line_id in &assignment.line_ids {
            if !eligible.contains(line_id) {
                return Err(StorageError::Database(sqlx::Error::Protocol(
                    "line not eligible for split (must be ordered, not anulada, and on parent)"
                        .into(),
                )));
            }
            let line_total: i64 = sqlx::query("SELECT total FROM document_details WHERE id = ?1")
                .bind(line_id.to_string())
                .fetch_one(&mut *tx)
                .await?
                .try_get("total")?;
            sqlx::query("UPDATE document_details SET document_id = ?1 WHERE id = ?2")
                .bind(child.id.to_string())
                .bind(line_id.to_string())
                .execute(&mut *tx)
                .await?;
            child_total += line_total;
        }
        sqlx::query("UPDATE documents SET total = ?1 WHERE id = ?2")
            .bind(child_total)
            .bind(child.id.to_string())
            .execute(&mut *tx)
            .await?;
        total_moved += child_total;
        created.push((child, child_total));
    }

    sqlx::query("UPDATE documents SET total = total - ?1 WHERE id = ?2")
        .bind(total_moved)
        .bind(parent_id.to_string())
        .execute(&mut *tx)
        .await?;

    if let Some(table_id) = parent.table_id {
        sqlx::query(
            "UPDATE mesa_estado SET subtotal_actual = subtotal_actual - ?1 WHERE mesa_id = ?2",
        )
        .bind(total_moved)
        .bind(table_id.to_string())
        .execute(&mut *tx)
        .await?;
    }

    // Se o pai ficou sem linhas, fecha-o operacionalmente (sem fiscal) e
    // liberta a mesa. Senão mantém-se aberto para receber pagamentos ou
    // novos pedidos das linhas que sobraram.
    let parent_remaining: i64 = sqlx::query("SELECT total FROM documents WHERE id = ?1")
        .bind(parent_id.to_string())
        .fetch_one(&mut *tx)
        .await?
        .try_get("total")?;
    if parent_remaining == 0 {
        sqlx::query("UPDATE documents SET is_closed = 1 WHERE id = ?1")
            .bind(parent_id.to_string())
            .execute(&mut *tx)
            .await?;
        if let Some(table_id) = parent.table_id {
            sqlx::query(
                "UPDATE mesa_estado SET estado = 'livre', empregado_actual_id = NULL, \
                 aberta_em = NULL, subtotal_actual = 0 WHERE mesa_id = ?1",
            )
            .bind(table_id.to_string())
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    // Materializa os filhos com totais persistidos.
    let mut out = Vec::with_capacity(created.len());
    for (child, _) in created {
        out.push(get_document(pool, child.id).await?);
    }
    Ok(out)
}

/// Insere uma linha (potencialmente fraccionária) num documento existente.
/// Não actualiza totais de documento/mesa — caller é responsável por o fazer
/// dentro da mesma transacção. Usado pelos modos Quantidades e Encaixar.
async fn insert_line_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    document_id: Uuid,
    article_id: Uuid,
    qty_milli: i64,
    unit_price: i64,
    total: i64,
    descricao: Option<&str>,
    pedida_em: Option<DateTime<Utc>>,
) -> Result<Uuid, StorageError> {
    let id = Uuid::new_v4();
    // qty inteiro é apenas para compatibilidade com leitores antigos; o que conta
    // é qty_milli e total. Usamos truncamento (qty_milli / 1000) — para shares
    // sub-unitários (e.g., 500) fica qty=0, o que é correcto.
    let qty_int = qty_milli / 1000;
    sqlx::query(
        "INSERT INTO document_details (id, document_id, article_id, qty, qty_milli, unit_price, \
         total, pedida_em, descricao) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )
    .bind(id.to_string())
    .bind(document_id.to_string())
    .bind(article_id.to_string())
    .bind(qty_int)
    .bind(qty_milli)
    .bind(unit_price)
    .bind(total)
    .bind(pedida_em)
    .bind(descricao)
    .execute(&mut **tx)
    .await?;
    Ok(id)
}

/// Modo **Quantidades**: divide cada linha elegível do pai em N partes iguais.
/// Cada filho recebe N linhas (uma por linha original) com `qty_milli` e
/// `total` proporcionais. Se uma linha L=151c e N=3, cada filho recebe 50c
/// (o cêntimo residual é absorvido pelo pai, que fica split-closed).
///
/// Garantia: **todas** as contas-filho ficam com o mesmo total, mesmo que a
/// soma fique 1c abaixo do total original.
pub async fn split_document_quantidades(
    pool: &SqlitePool,
    parent_id: Uuid,
    num_accounts: usize,
) -> Result<Vec<Document>, StorageError> {
    if num_accounts < 2 {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "split requires at least 2 accounts".into(),
        )));
    }
    let parent = get_document(pool, parent_id).await?;
    if parent.is_closed {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "parent document already closed".into(),
        )));
    }
    let lines = list_document_details(pool, parent_id).await?;
    let eligible: Vec<&DocumentDetail> = lines
        .iter()
        .filter(|l| l.pedida_em.is_some() && !l.anulada)
        .collect();
    if eligible.is_empty() {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "no eligible lines to split".into(),
        )));
    }

    let mut tx = pool.begin().await?;
    let mut children = Vec::with_capacity(num_accounts);
    for _ in 0..num_accounts {
        children.push(create_child_document_tx(&mut tx, &parent).await?);
    }

    let n = num_accounts as i64;
    let mut child_totals: Vec<i64> = vec![0; num_accounts];
    let mut moved_total: i64 = 0;

    for line in &eligible {
        // Cada conta recebe `share` cêntimos da linha. O resto (line.total -
        // n*share) é absorvido pelo pai. Igualmente para qty_milli.
        let share_cents = line.total / n;
        let share_milli = line.qty_milli / n;
        if share_cents <= 0 {
            // Linha cobra menos de 1c por conta — não dá para representar como
            // share igual entre contas com inteiros. Salta-a (fica no pai).
            continue;
        }
        for (idx, child) in children.iter().enumerate() {
            insert_line_tx(
                &mut tx,
                child.id,
                line.article_id,
                share_milli,
                line.unit_price,
                share_cents,
                None,
                line.pedida_em,
            )
            .await?;
            child_totals[idx] += share_cents;
        }
        moved_total += n * share_cents;
        // Apaga a linha original do pai (já redistribuída).
        sqlx::query("DELETE FROM document_details WHERE id = ?1")
            .bind(line.id.to_string())
            .execute(&mut *tx)
            .await?;
    }

    for (child, total) in children.iter().zip(child_totals.iter()) {
        sqlx::query("UPDATE documents SET total = ?1 WHERE id = ?2")
            .bind(*total)
            .bind(child.id.to_string())
            .execute(&mut *tx)
            .await?;
    }

    // Pai: total = total residual (parcela perdida ao arredondar). Marca-o
    // operacionalmente fechado e liberta a mesa.
    sqlx::query("UPDATE documents SET total = total - ?1, is_closed = 1 WHERE id = ?2")
        .bind(moved_total)
        .bind(parent_id.to_string())
        .execute(&mut *tx)
        .await?;
    if let Some(table_id) = parent.table_id {
        sqlx::query(
            "UPDATE mesa_estado SET estado = 'livre', empregado_actual_id = NULL, \
             aberta_em = NULL, subtotal_actual = 0 WHERE mesa_id = ?1",
        )
        .bind(table_id.to_string())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    let mut out = Vec::with_capacity(children.len());
    for child in children {
        out.push(get_document(pool, child.id).await?);
    }
    Ok(out)
}

/// Modo **Encaixar**: o operador atribui as linhas elegíveis a contas
/// "primárias" (via `assignments`). Cada conta acaba com `target = total/N`,
/// graças a linhas de compensação positivas/negativas geradas pelo sistema.
///
/// Para cada linha L atribuída à conta A, gera:
///   * em A: -share por cada outra conta B (total -L.total*(N-1)/N)
///   * em cada outra B: +share
///
/// Cêntimos residuais (quando L.total não divide por N) são absorvidos pelo
/// pai — garantia: todas as contas-filho têm exactamente o mesmo total.
pub async fn split_document_encaixar(
    pool: &SqlitePool,
    parent_id: Uuid,
    assignments: &[SplitAssignment],
) -> Result<Vec<Document>, StorageError> {
    if assignments.len() < 2 {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "encaixar requires at least 2 accounts".into(),
        )));
    }
    let parent = get_document(pool, parent_id).await?;
    if parent.is_closed {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "parent document already closed".into(),
        )));
    }
    let parent_lines = list_document_details(pool, parent_id).await?;
    let n = assignments.len() as i64;

    // Map line_id -> (line, assigned account index)
    let mut line_owner: std::collections::HashMap<Uuid, usize> = std::collections::HashMap::new();
    for (idx, a) in assignments.iter().enumerate() {
        for lid in &a.line_ids {
            if line_owner.insert(*lid, idx).is_some() {
                return Err(StorageError::Database(sqlx::Error::Protocol(
                    "line assigned to more than one account".into(),
                )));
            }
        }
    }
    // Valida elegibilidade
    let mut eligible_lines: Vec<&DocumentDetail> = Vec::new();
    for (lid, _) in &line_owner {
        let line = parent_lines
            .iter()
            .find(|l| l.id == *lid)
            .ok_or(StorageError::NotFound)?;
        if line.pedida_em.is_none() || line.anulada {
            return Err(StorageError::Database(sqlx::Error::Protocol(
                "line not eligible for encaixar (must be ordered, not anulada)".into(),
            )));
        }
        eligible_lines.push(line);
    }
    if eligible_lines.is_empty() {
        return Err(StorageError::Database(sqlx::Error::Protocol(
            "no eligible lines for encaixar".into(),
        )));
    }

    let mut tx = pool.begin().await?;
    let mut children = Vec::with_capacity(assignments.len());
    for _ in 0..assignments.len() {
        children.push(create_child_document_tx(&mut tx, &parent).await?);
    }

    let mut child_totals: Vec<i64> = vec![0; assignments.len()];
    let mut moved_total: i64 = 0;

    for line in &eligible_lines {
        let owner_idx = line_owner[&line.id];
        // Cada outra conta recebe share = floor(line.total / N). O cêntimo
        // residual é perdido no pai (mantém-se a invariante de igualdade).
        let share = line.total / n;
        if share <= 0 {
            // Linha não amortizável; deixa no pai.
            continue;
        }
        // Mover a linha original para a conta primária.
        sqlx::query("UPDATE document_details SET document_id = ?1 WHERE id = ?2")
            .bind(children[owner_idx].id.to_string())
            .bind(line.id.to_string())
            .execute(&mut *tx)
            .await?;
        child_totals[owner_idx] += line.total;
        moved_total += line.total;

        // Resolve a descrição da compensação a partir do nome do artigo.
        let article_name: String = sqlx::query("SELECT name FROM articles WHERE id = ?1")
            .bind(line.article_id.to_string())
            .fetch_one(&mut *tx)
            .await?
            .try_get("name")?;
        let comp_desc = format!("Compensação {}", article_name);

        // Para cada outra conta: insere +share lá e -share na primária.
        let share_milli = line.qty_milli / n; // proporcional, informativo
        for other_idx in 0..assignments.len() {
            if other_idx == owner_idx {
                continue;
            }
            insert_line_tx(
                &mut tx,
                children[other_idx].id,
                line.article_id,
                share_milli,
                line.unit_price,
                share,
                Some(&comp_desc),
                line.pedida_em,
            )
            .await?;
            child_totals[other_idx] += share;

            insert_line_tx(
                &mut tx,
                children[owner_idx].id,
                line.article_id,
                -share_milli,
                line.unit_price,
                -share,
                Some(&comp_desc),
                line.pedida_em,
            )
            .await?;
            child_totals[owner_idx] -= share;
        }
    }

    for (child, total) in children.iter().zip(child_totals.iter()) {
        sqlx::query("UPDATE documents SET total = ?1 WHERE id = ?2")
            .bind(*total)
            .bind(child.id.to_string())
            .execute(&mut *tx)
            .await?;
    }

    // Pai: subtrai o que foi movido; marca-o split-closed e liberta a mesa.
    sqlx::query("UPDATE documents SET total = total - ?1, is_closed = 1 WHERE id = ?2")
        .bind(moved_total)
        .bind(parent_id.to_string())
        .execute(&mut *tx)
        .await?;
    if let Some(table_id) = parent.table_id {
        sqlx::query(
            "UPDATE mesa_estado SET estado = 'livre', empregado_actual_id = NULL, \
             aberta_em = NULL, subtotal_actual = 0 WHERE mesa_id = ?1",
        )
        .bind(table_id.to_string())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    let mut out = Vec::with_capacity(children.len());
    for child in children {
        out.push(get_document(pool, child.id).await?);
    }
    Ok(out)
}

/// Heurística greedy (LPT) — atribui cada linha (ordenada por total decrescente)
/// ao filho com o menor total acumulado. Minimiza a diferença máxima entre
/// contas. Devolve as `SplitAssignment` prontas para serem passadas a
/// `split_document`. Não muta a BD: a UI pode mostrar a sugestão e deixar o
/// utilizador ajustar antes de confirmar.
pub fn plan_auto_split(lines: &[DocumentDetail], num_accounts: usize) -> Vec<SplitAssignment> {
    assert!(num_accounts > 0, "num_accounts must be positive");
    let mut buckets: Vec<(i64, Vec<Uuid>)> = (0..num_accounts).map(|_| (0, Vec::new())).collect();
    let mut sorted: Vec<&DocumentDetail> = lines
        .iter()
        .filter(|l| l.pedida_em.is_some() && !l.anulada)
        .collect();
    sorted.sort_by(|a, b| b.total.cmp(&a.total));
    for line in sorted {
        let (idx, _) = buckets
            .iter()
            .enumerate()
            .min_by_key(|(i, (total, _))| (*total, *i))
            .expect("at least one bucket");
        buckets[idx].0 += line.total;
        buckets[idx].1.push(line.id);
    }
    buckets
        .into_iter()
        .map(|(_, line_ids)| SplitAssignment { line_ids })
        .collect()
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

const CUSTOMER_COLS: &str = "id, codigo, nome, nif, pais, telefone, morada, cod_postal, \
        localidade, email, observacoes, numero_cartao, limite_credito, zona_id, anulado_em, \
        esquecido_em";

fn customer_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Customer, StorageError> {
    let zona_id: Option<String> = r.try_get("zona_id")?;
    Ok(Customer {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        codigo: r.try_get::<Option<i64>, _>("codigo")?.map(|v| v as i32),
        nome: r.try_get("nome")?,
        nif: r.try_get("nif")?,
        pais: r.try_get("pais")?,
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
        esquecido_em: r.try_get::<Option<DateTime<Utc>>, _>("esquecido_em")?,
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
    pub pais: Option<String>,
    pub telefone: Option<String>,
    pub morada: Option<String>,
    pub cod_postal: Option<String>,
    pub localidade: Option<String>,
    pub email: Option<String>,
    pub observacoes: Option<String>,
    pub zona_id: Option<Uuid>,
}

pub async fn create_customer(
    pool: &SqlitePool,
    input: NewCustomer,
) -> Result<Customer, StorageError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO clientes (id, nome, nif, pais, telefone, morada, cod_postal, localidade, \
         email, observacoes, zona_id) \
         VALUES (?1, ?2, ?3, COALESCE(?4, 'PT'), ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
    )
    .bind(id.to_string())
    .bind(&input.nome)
    .bind(input.nif.as_deref())
    .bind(input.pais.as_deref())
    .bind(input.telefone.as_deref())
    .bind(input.morada.as_deref())
    .bind(input.cod_postal.as_deref())
    .bind(input.localidade.as_deref())
    .bind(input.email.as_deref())
    .bind(input.observacoes.as_deref())
    .bind(input.zona_id.map(|u| u.to_string()))
    .execute(pool)
    .await?;
    get_customer(pool, id).await
}

#[derive(Default)]
pub struct CustomerUpdate {
    pub nome: Option<String>,
    pub nif: Option<Option<String>>,
    pub pais: Option<String>,
    pub telefone: Option<Option<String>>,
    pub morada: Option<Option<String>>,
    pub cod_postal: Option<Option<String>>,
    pub localidade: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub observacoes: Option<Option<String>>,
    pub zona_id: Option<Option<Uuid>>,
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
    if let Some(v) = upd.pais {
        if !first { q.push(", "); }
        q.push("pais = ");
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
    if let Some(v) = upd.zona_id {
        if !first { q.push(", "); }
        q.push("zona_id = ");
        q.push_bind(v.map(|u| u.to_string()));
        first = false;
    }

    if first {
        return get_customer(pool, id).await;
    }
    q.push(" WHERE id = ").push_bind(id.to_string());
    q.build().execute(pool).await?;
    get_customer(pool, id).await
}

/// RGPD "right to be forgotten": anonimiza dados pessoais mas preserva o
/// registo (mantém id, codigo e ligações fiscais para SAF-T histórico).
/// Idempotente — chamada repetida não muda nada.
pub async fn forget_customer(pool: &SqlitePool, id: Uuid) -> Result<Customer, StorageError> {
    let cur = get_customer(pool, id).await?;
    if cur.esquecido_em.is_some() {
        return Ok(cur);
    }
    let now = Utc::now();
    sqlx::query(
        "UPDATE clientes SET nome = '[ESQUECIDO]', nif = NULL, telefone = NULL, \
         morada = NULL, cod_postal = NULL, localidade = NULL, email = NULL, \
         observacoes = NULL, numero_cartao = NULL, esquecido_em = ?1, \
         anulado_em = COALESCE(anulado_em, ?1) WHERE id = ?2",
    )
    .bind(now)
    .bind(id.to_string())
    .execute(pool)
    .await?;
    get_customer(pool, id).await
}

const DELIVERY_COLS: &str = "id, document_id, cliente_id, morada_snapshot, telefone_snapshot, \
        recebido_em, recebido_via, entregador_id, pronto_em, despachado_em, entregue_em, estado, \
        zona_id, taxa_entrega_cents";

fn delivery_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<PedidoDelivery, StorageError> {
    let cliente_id: Option<String> = r.try_get("cliente_id")?;
    let entregador_id: Option<String> = r.try_get("entregador_id")?;
    let zona_id: Option<String> = r.try_get("zona_id")?;
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
        zona_id: parse_optional_uuid(zona_id)?,
        taxa_entrega_cents: r.try_get::<i64, _>("taxa_entrega_cents")?,
    })
}

pub async fn create_pedido_delivery(
    pool: &SqlitePool,
    document_id: Uuid,
    cliente_id: Option<Uuid>,
    morada: Option<String>,
    telefone: Option<String>,
    recebido_via: &str,
    zona_id: Option<Uuid>,
    taxa_entrega_cents: i64,
) -> Result<PedidoDelivery, StorageError> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO pedidos_delivery (id, document_id, cliente_id, morada_snapshot, \
         telefone_snapshot, recebido_em, recebido_via, estado, zona_id, taxa_entrega_cents) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'recebido', ?8, ?9)",
    )
    .bind(id.to_string())
    .bind(document_id.to_string())
    .bind(cliente_id.map(|u| u.to_string()))
    .bind(morada.as_deref())
    .bind(telefone.as_deref())
    .bind(now)
    .bind(recebido_via)
    .bind(zona_id.map(|u| u.to_string()))
    .bind(taxa_entrega_cents)
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

pub async fn list_tipos_preco(pool: &SqlitePool) -> Result<Vec<TipoPreco>, StorageError> {
    let rows = sqlx::query("SELECT id, codigo, designacao FROM tipos_preco ORDER BY codigo")
        .fetch_all(pool)
        .await?;
    rows.into_iter()
        .map(|r| {
            Ok(TipoPreco {
                id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
                codigo: r.try_get::<i64, _>("codigo")? as i32,
                designacao: r.try_get("designacao")?,
            })
        })
        .collect()
}

pub async fn get_tipo_preco(pool: &SqlitePool, id: Uuid) -> Result<TipoPreco, StorageError> {
    let r = sqlx::query("SELECT id, codigo, designacao FROM tipos_preco WHERE id = ?1")
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    Ok(TipoPreco {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        codigo: r.try_get::<i64, _>("codigo")? as i32,
        designacao: r.try_get("designacao")?,
    })
}

/// Devolve o preço do artigo para um dado local: usa o pvp do tipo_preco do local,
/// com fallback para pvp1 quando o local não tem tipo_preco configurado ou o pvp escolhido é 0.
pub async fn price_for_local(
    pool: &SqlitePool,
    article: &Article,
    local: &Local,
) -> Result<i64, StorageError> {
    let codigo = if let Some(tid) = local.tipo_preco_id {
        get_tipo_preco(pool, tid).await?.codigo
    } else {
        1
    };
    Ok(article.pvp_for(codigo))
}

const ZONA_COLS: &str = "id, codigo, designacao, taxa_entrega, rede_remota_associada_id, anulado_em";

fn zona_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Zona, StorageError> {
    let rede: Option<String> = r.try_get("rede_remota_associada_id")?;
    Ok(Zona {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        codigo: r.try_get::<Option<i64>, _>("codigo")?.map(|v| v as i32),
        designacao: r.try_get("designacao")?,
        taxa_entrega: r.try_get::<i64, _>("taxa_entrega")?,
        rede_remota_associada_id: parse_optional_uuid(rede)?,
        anulado_em: r.try_get::<Option<DateTime<Utc>>, _>("anulado_em")?,
    })
}

pub async fn list_zonas(pool: &SqlitePool) -> Result<Vec<Zona>, StorageError> {
    let q = format!("SELECT {ZONA_COLS} FROM zonas WHERE anulado_em IS NULL ORDER BY designacao");
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(zona_from_row).collect()
}

pub async fn get_zona(pool: &SqlitePool, id: Uuid) -> Result<Zona, StorageError> {
    let q = format!("SELECT {ZONA_COLS} FROM zonas WHERE id = ?1");
    let r = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    zona_from_row(&r)
}

pub struct NewZona {
    pub designacao: String,
    pub codigo: Option<i32>,
    pub taxa_entrega: i64,
}

pub async fn create_zona(pool: &SqlitePool, input: NewZona) -> Result<Zona, StorageError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO zonas (id, codigo, designacao, taxa_entrega) VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(id.to_string())
    .bind(input.codigo.map(|v| v as i64))
    .bind(&input.designacao)
    .bind(input.taxa_entrega)
    .execute(pool)
    .await?;
    get_zona(pool, id).await
}

#[derive(Default)]
pub struct ZonaUpdate {
    pub designacao: Option<String>,
    pub codigo: Option<Option<i32>>,
    pub taxa_entrega: Option<i64>,
}

pub async fn update_zona(
    pool: &SqlitePool,
    id: Uuid,
    upd: ZonaUpdate,
) -> Result<Zona, StorageError> {
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE zonas SET ");
    let mut first = true;
    if let Some(v) = upd.designacao {
        q.push("designacao = ").push_bind(v);
        first = false;
    }
    if let Some(v) = upd.codigo {
        if !first { q.push(", "); }
        q.push("codigo = ").push_bind(v.map(|x| x as i64));
        first = false;
    }
    if let Some(v) = upd.taxa_entrega {
        if !first { q.push(", "); }
        q.push("taxa_entrega = ").push_bind(v);
        first = false;
    }
    if first {
        return get_zona(pool, id).await;
    }
    q.push(" WHERE id = ").push_bind(id.to_string());
    q.build().execute(pool).await?;
    get_zona(pool, id).await
}

pub async fn delete_zona(pool: &SqlitePool, id: Uuid) -> Result<(), StorageError> {
    sqlx::query("UPDATE zonas SET anulado_em = ?1 WHERE id = ?2")
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

const ENTREGADOR_COLS: &str = "id, nome, telefone, externo, ativo, anulado_em";

fn entregador_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Entregador, StorageError> {
    Ok(Entregador {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        nome: r.try_get("nome")?,
        telefone: r.try_get("telefone")?,
        externo: r.try_get::<bool, _>("externo")?,
        ativo: r.try_get::<bool, _>("ativo")?,
        anulado_em: r.try_get::<Option<DateTime<Utc>>, _>("anulado_em")?,
    })
}

pub async fn list_entregadores(pool: &SqlitePool) -> Result<Vec<Entregador>, StorageError> {
    let q = format!(
        "SELECT {ENTREGADOR_COLS} FROM entregadores WHERE anulado_em IS NULL ORDER BY nome"
    );
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(entregador_from_row).collect()
}

pub async fn get_entregador(pool: &SqlitePool, id: Uuid) -> Result<Entregador, StorageError> {
    let q = format!("SELECT {ENTREGADOR_COLS} FROM entregadores WHERE id = ?1");
    let r = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    entregador_from_row(&r)
}

pub struct NewEntregador {
    pub nome: String,
    pub telefone: Option<String>,
    pub externo: bool,
}

pub async fn create_entregador(
    pool: &SqlitePool,
    input: NewEntregador,
) -> Result<Entregador, StorageError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO entregadores (id, nome, telefone, externo, ativo) VALUES (?1, ?2, ?3, ?4, 1)",
    )
    .bind(id.to_string())
    .bind(&input.nome)
    .bind(input.telefone.as_deref())
    .bind(input.externo as i32)
    .execute(pool)
    .await?;
    get_entregador(pool, id).await
}

#[derive(Default)]
pub struct EntregadorUpdate {
    pub nome: Option<String>,
    pub telefone: Option<Option<String>>,
    pub externo: Option<bool>,
    pub ativo: Option<bool>,
}

pub async fn update_entregador(
    pool: &SqlitePool,
    id: Uuid,
    upd: EntregadorUpdate,
) -> Result<Entregador, StorageError> {
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE entregadores SET ");
    let mut first = true;
    if let Some(v) = upd.nome {
        q.push("nome = ").push_bind(v);
        first = false;
    }
    if let Some(v) = upd.telefone {
        if !first { q.push(", "); }
        q.push("telefone = ").push_bind(v);
        first = false;
    }
    if let Some(v) = upd.externo {
        if !first { q.push(", "); }
        q.push("externo = ").push_bind(v as i32);
        first = false;
    }
    if let Some(v) = upd.ativo {
        if !first { q.push(", "); }
        q.push("ativo = ").push_bind(v as i32);
        first = false;
    }
    if first {
        return get_entregador(pool, id).await;
    }
    q.push(" WHERE id = ").push_bind(id.to_string());
    q.build().execute(pool).await?;
    get_entregador(pool, id).await
}

pub async fn delete_entregador(pool: &SqlitePool, id: Uuid) -> Result<(), StorageError> {
    sqlx::query("UPDATE entregadores SET anulado_em = ?1, ativo = 0 WHERE id = ?2")
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

const DEVICE_COLS: &str = "id, nome, tipo, modelo, descricao, output_path, ativo, anulado_em";

fn dispositivo_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<Dispositivo, StorageError> {
    Ok(Dispositivo {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        nome: r.try_get("nome")?,
        tipo: r.try_get("tipo")?,
        modelo: r.try_get("modelo")?,
        descricao: r.try_get("descricao")?,
        output_path: r.try_get("output_path")?,
        ativo: r.try_get::<bool, _>("ativo")?,
        anulado_em: r.try_get::<Option<DateTime<Utc>>, _>("anulado_em")?,
    })
}

pub async fn list_dispositivos(pool: &SqlitePool) -> Result<Vec<Dispositivo>, StorageError> {
    let q = format!(
        "SELECT {DEVICE_COLS} FROM dispositivos WHERE anulado_em IS NULL ORDER BY nome"
    );
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(dispositivo_from_row).collect()
}

pub async fn get_dispositivo(pool: &SqlitePool, id: Uuid) -> Result<Dispositivo, StorageError> {
    let q = format!("SELECT {DEVICE_COLS} FROM dispositivos WHERE id = ?1");
    let r = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    dispositivo_from_row(&r)
}

pub struct NewDispositivo {
    pub nome: String,
    pub tipo: String,
    pub modelo: Option<String>,
    pub descricao: Option<String>,
    pub output_path: Option<String>,
}

pub async fn create_dispositivo(
    pool: &SqlitePool,
    input: NewDispositivo,
) -> Result<Dispositivo, StorageError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO dispositivos (id, nome, tipo, modelo, descricao, output_path, ativo) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1)",
    )
    .bind(id.to_string())
    .bind(&input.nome)
    .bind(&input.tipo)
    .bind(input.modelo.as_deref())
    .bind(input.descricao.as_deref())
    .bind(input.output_path.as_deref())
    .execute(pool)
    .await?;
    get_dispositivo(pool, id).await
}

#[derive(Default)]
pub struct DispositivoUpdate {
    pub nome: Option<String>,
    pub tipo: Option<String>,
    pub modelo: Option<Option<String>>,
    pub descricao: Option<Option<String>>,
    pub output_path: Option<Option<String>>,
    pub ativo: Option<bool>,
}

pub async fn update_dispositivo(
    pool: &SqlitePool,
    id: Uuid,
    upd: DispositivoUpdate,
) -> Result<Dispositivo, StorageError> {
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE dispositivos SET ");
    let mut first = true;
    macro_rules! set {
        ($field:ident, $col:literal) => {
            if let Some(v) = upd.$field {
                if !first { q.push(", "); }
                q.push($col).push(" = ");
                q.push_bind(v);
                first = false;
            }
        };
    }
    set!(nome, "nome");
    set!(tipo, "tipo");
    set!(modelo, "modelo");
    set!(descricao, "descricao");
    set!(output_path, "output_path");
    if let Some(v) = upd.ativo {
        if !first { q.push(", "); }
        q.push("ativo = ").push_bind(v as i32);
        first = false;
    }
    if first {
        return get_dispositivo(pool, id).await;
    }
    q.push(" WHERE id = ").push_bind(id.to_string());
    q.build().execute(pool).await?;
    get_dispositivo(pool, id).await
}

pub async fn delete_dispositivo(pool: &SqlitePool, id: Uuid) -> Result<(), StorageError> {
    sqlx::query("UPDATE dispositivos SET anulado_em = ?1, ativo = 0 WHERE id = ?2")
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

const ZONA_IMP_COLS: &str = "id, codigo, designacao, secundarios, anulado_em";

fn zona_imp_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<ZonaImpressao, StorageError> {
    Ok(ZonaImpressao {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        codigo: r.try_get::<i64, _>("codigo")? as i32,
        designacao: r.try_get("designacao")?,
        secundarios: r.try_get::<bool, _>("secundarios")?,
        anulado_em: r.try_get::<Option<DateTime<Utc>>, _>("anulado_em")?,
    })
}

pub async fn list_zonas_impressao(pool: &SqlitePool) -> Result<Vec<ZonaImpressao>, StorageError> {
    let q = format!(
        "SELECT {ZONA_IMP_COLS} FROM zonas_impressao WHERE anulado_em IS NULL ORDER BY codigo"
    );
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(zona_imp_from_row).collect()
}

pub async fn get_zona_impressao(
    pool: &SqlitePool,
    id: Uuid,
) -> Result<ZonaImpressao, StorageError> {
    let q = format!("SELECT {ZONA_IMP_COLS} FROM zonas_impressao WHERE id = ?1");
    let r = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or(StorageError::NotFound)?;
    zona_imp_from_row(&r)
}

pub struct NewZonaImpressao {
    pub codigo: i32,
    pub designacao: String,
    pub secundarios: bool,
}

pub async fn create_zona_impressao(
    pool: &SqlitePool,
    input: NewZonaImpressao,
) -> Result<ZonaImpressao, StorageError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO zonas_impressao (id, codigo, designacao, secundarios) \
         VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(id.to_string())
    .bind(input.codigo as i64)
    .bind(&input.designacao)
    .bind(input.secundarios as i32)
    .execute(pool)
    .await?;
    get_zona_impressao(pool, id).await
}

#[derive(Default)]
pub struct ZonaImpressaoUpdate {
    pub codigo: Option<i32>,
    pub designacao: Option<String>,
    pub secundarios: Option<bool>,
}

pub async fn update_zona_impressao(
    pool: &SqlitePool,
    id: Uuid,
    upd: ZonaImpressaoUpdate,
) -> Result<ZonaImpressao, StorageError> {
    let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE zonas_impressao SET ");
    let mut first = true;
    if let Some(v) = upd.codigo {
        q.push("codigo = ").push_bind(v as i64);
        first = false;
    }
    if let Some(v) = upd.designacao {
        if !first { q.push(", "); }
        q.push("designacao = ").push_bind(v);
        first = false;
    }
    if let Some(v) = upd.secundarios {
        if !first { q.push(", "); }
        q.push("secundarios = ").push_bind(v as i32);
        first = false;
    }
    if first {
        return get_zona_impressao(pool, id).await;
    }
    q.push(" WHERE id = ").push_bind(id.to_string());
    q.build().execute(pool).await?;
    get_zona_impressao(pool, id).await
}

pub async fn delete_zona_impressao(pool: &SqlitePool, id: Uuid) -> Result<(), StorageError> {
    sqlx::query("UPDATE zonas_impressao SET anulado_em = ?1 WHERE id = ?2")
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

const MAPPING_COLS: &str = "id, zona_impressao_id, local_id, origem_id, dispositivo_id, \
        agrupamento, numero_copias";

fn mapping_from_row(r: &sqlx::sqlite::SqliteRow) -> Result<ImpressoraZonaLocal, StorageError> {
    let origem: Option<String> = r.try_get("origem_id")?;
    Ok(ImpressoraZonaLocal {
        id: Uuid::parse_str(r.try_get::<&str, _>("id")?)?,
        zona_impressao_id: Uuid::parse_str(r.try_get::<&str, _>("zona_impressao_id")?)?,
        local_id: Uuid::parse_str(r.try_get::<&str, _>("local_id")?)?,
        origem_id: parse_optional_uuid(origem)?,
        dispositivo_id: Uuid::parse_str(r.try_get::<&str, _>("dispositivo_id")?)?,
        agrupamento: r.try_get("agrupamento")?,
        numero_copias: r.try_get::<i64, _>("numero_copias")? as i32,
    })
}

pub async fn list_print_mappings(
    pool: &SqlitePool,
) -> Result<Vec<ImpressoraZonaLocal>, StorageError> {
    let q = format!("SELECT {MAPPING_COLS} FROM impressora_zona_local");
    let rows = sqlx::query(&q).fetch_all(pool).await?;
    rows.iter().map(mapping_from_row).collect()
}

/// Devolve o dispositivo que serve uma zona num dado local.
pub async fn dispositivo_for_zona_local(
    pool: &SqlitePool,
    zona_impressao_id: Uuid,
    local_id: Uuid,
) -> Result<Option<Dispositivo>, StorageError> {
    let q = format!(
        "SELECT d.{} FROM impressora_zona_local m \
         JOIN dispositivos d ON d.id = m.dispositivo_id \
         WHERE m.zona_impressao_id = ?1 AND m.local_id = ?2 \
         ORDER BY m.origem_id IS NULL LIMIT 1",
        DEVICE_COLS.replace(", ", ", d.")
    );
    let row = sqlx::query(&q)
        .bind(zona_impressao_id.to_string())
        .bind(local_id.to_string())
        .fetch_optional(pool)
        .await?;
    row.as_ref().map(dispositivo_from_row).transpose()
}

pub struct NewMapping {
    pub zona_impressao_id: Uuid,
    pub local_id: Uuid,
    pub origem_id: Option<Uuid>,
    pub dispositivo_id: Uuid,
    pub agrupamento: String,
    pub numero_copias: i32,
}

pub async fn create_print_mapping(
    pool: &SqlitePool,
    input: NewMapping,
) -> Result<ImpressoraZonaLocal, StorageError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO impressora_zona_local (id, zona_impressao_id, local_id, origem_id, \
         dispositivo_id, agrupamento, numero_copias) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )
    .bind(id.to_string())
    .bind(input.zona_impressao_id.to_string())
    .bind(input.local_id.to_string())
    .bind(input.origem_id.map(|u| u.to_string()))
    .bind(input.dispositivo_id.to_string())
    .bind(&input.agrupamento)
    .bind(input.numero_copias as i64)
    .execute(pool)
    .await?;
    let q = format!("SELECT {MAPPING_COLS} FROM impressora_zona_local WHERE id = ?1");
    let row = sqlx::query(&q)
        .bind(id.to_string())
        .fetch_one(pool)
        .await?;
    mapping_from_row(&row)
}

pub async fn delete_print_mapping(pool: &SqlitePool, id: Uuid) -> Result<(), StorageError> {
    sqlx::query("DELETE FROM impressora_zona_local WHERE id = ?1")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

/// Insere ou actualiza o ATCUD (código de validação de série) para a tupla
/// `(document_type, series_prefix, year)`. Marca quaisquer ATCUDs anteriores
/// dessa tupla como `is_active=0` — garante que apenas uma entrada activa
/// existe por série/ano (a última obtida da AT).
pub async fn upsert_atcud(
    pool: &SqlitePool,
    document_type: &str,
    series_prefix: &str,
    year: i32,
    atcud: &str,
    start_date: NaiveDate,
) -> Result<Atcud, StorageError> {
    let mut tx = pool.begin().await?;
    sqlx::query(
        "UPDATE atcud SET is_active = 0 \
         WHERE document_type = ?1 AND series_prefix = ?2 AND year = ?3 AND is_active = 1",
    )
    .bind(document_type)
    .bind(series_prefix)
    .bind(year as i64)
    .execute(&mut *tx)
    .await?;

    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO atcud (id, document_type, series_prefix, year, atcud, start_date, \
         registered_at, is_active) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1)",
    )
    .bind(id.to_string())
    .bind(document_type)
    .bind(series_prefix)
    .bind(year as i64)
    .bind(atcud)
    .bind(start_date)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Atcud {
        id,
        document_type: document_type.into(),
        series_prefix: series_prefix.into(),
        year,
        atcud: atcud.into(),
        start_date,
        registered_at: now,
        is_active: true,
    })
}

/// Marca o ATCUD activo de uma série como inactivo (sem o apagar — mantém
/// histórico). Usado após `finalizarSerie` ou `anularSerie` na AT.
pub async fn deactivate_atcud(
    pool: &SqlitePool,
    document_type: &str,
    series_prefix: &str,
    year: i32,
) -> Result<(), StorageError> {
    sqlx::query(
        "UPDATE atcud SET is_active = 0 \
         WHERE document_type = ?1 AND series_prefix = ?2 AND year = ?3 AND is_active = 1",
    )
    .bind(document_type)
    .bind(series_prefix)
    .bind(year as i64)
    .execute(pool)
    .await?;
    Ok(())
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
