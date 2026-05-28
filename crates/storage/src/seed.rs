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

    // Tipos de preço (spec 3.1)
    let tp_mesa = Uuid::new_v4();
    let tp_takeaway = Uuid::new_v4();
    let tp_festa = Uuid::new_v4();
    let tp_delivery = Uuid::new_v4();
    let tp_pessoal = Uuid::new_v4();
    for (id, codigo, nome) in [
        (tp_mesa, 1, "Mesa"),
        (tp_takeaway, 2, "Take-Away"),
        (tp_festa, 3, "Festa"),
        (tp_delivery, 4, "Delivery"),
        (tp_pessoal, 5, "Pessoal"),
    ] {
        sqlx::query("INSERT INTO tipos_preco (id, codigo, designacao) VALUES (?1, ?2, ?3)")
            .bind(id.to_string())
            .bind(codigo as i64)
            .bind(nome)
            .execute(&mut *tx)
            .await?;
    }

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

    // Zonas de impressão (spec 10.3) — Cozinha e Bar marcadas como secundárias
    // para receberem o bloco "sai junto com" do pedido cruzado.
    let zi_caixa = Uuid::new_v4();
    let zi_cozinha = Uuid::new_v4();
    let zi_bar = Uuid::new_v4();
    for (id, codigo, designacao, secundarios) in [
        (zi_caixa, 1, "Documentos Externos", false),
        (zi_cozinha, 10, "Cozinha", true),
        (zi_bar, 11, "Bar", true),
    ] {
        sqlx::query(
            "INSERT INTO zonas_impressao (id, codigo, designacao, secundarios) \
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(id.to_string())
        .bind(codigo as i64)
        .bind(designacao)
        .bind(secundarios as i32)
        .execute(&mut *tx)
        .await?;
    }

    // Dispositivos: 3 impressoras file-based para demo.
    let imp_caixa = Uuid::new_v4();
    let imp_cozinha = Uuid::new_v4();
    let imp_bar = Uuid::new_v4();
    for (id, nome, path) in [
        (imp_caixa, "Impressora Caixa", "./receipts.txt"),
        (imp_cozinha, "Impressora Cozinha", "./cozinha.txt"),
        (imp_bar, "Impressora Bar", "./bar.txt"),
    ] {
        sqlx::query(
            "INSERT INTO dispositivos (id, nome, tipo, output_path, ativo) \
             VALUES (?1, ?2, 'impressora_generica', ?3, 1)",
        )
        .bind(id.to_string())
        .bind(nome)
        .bind(path)
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

    // Articles — (code, name, family, vat, pvp1, pvp5, zona_impressao)
    let articles: &[(i64, &str, Uuid, i64, i64, i64, Uuid)] = &[
        (1, "Café Expresso", sub_cafes, 1300, 80, 0, zi_bar),
        (2, "Meia de Leite", sub_cafes, 1300, 110, 0, zi_bar),
        (3, "Galão", sub_cafes, 1300, 150, 0, zi_bar),
        (4, "Descafeinado", sub_cafes, 1300, 90, 0, zi_bar),
        (10, "Tosta Mista", sub_tostas, 1300, 300, 150, zi_cozinha),
        (11, "Sandes Fiambre", sub_tostas, 1300, 250, 100, zi_cozinha),
        (12, "Croissant", sub_pastel, 1300, 150, 80, zi_cozinha),
        (13, "Pastel de Nata", sub_pastel, 1300, 130, 60, zi_cozinha),
        (20, "Água 33cl", sub_aguas, 1300, 120, 0, zi_bar),
        (21, "Sumo Laranja", sub_refri, 1300, 250, 120, zi_bar),
        (22, "Coca-Cola 33cl", sub_refri, 2300, 230, 100, zi_bar),
        (23, "Cerveja 33cl", sub_refri, 2300, 220, 100, zi_bar),
    ];
    for (code, name, family_id, vat, pvp1, pvp5, zona) in articles {
        sqlx::query(
            "INSERT INTO articles (id, family_id, code, name, pvp1, pvp5, vat_rate, \
             zona_impressao_id, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(family_id.to_string())
        .bind(code)
        .bind(name)
        .bind(pvp1)
        .bind(pvp5)
        .bind(vat)
        .bind(zona.to_string())
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    // Artigo "gorjeta" automático para taxa de entrega (delivery).
    let art_taxa_entrega = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO articles (id, code, name, pvp1, vat_rate, tipo_artigo, created_at, updated_at) \
         VALUES (?1, 9999, 'Taxa de Entrega', 0, 2300, 'gorjeta', ?2, ?2)",
    )
    .bind(art_taxa_entrega.to_string())
    .bind(now)
    .execute(&mut *tx)
    .await?;

    // Local default "Salão Principal" — modo normal, 12 mesas. Usa PVP1 (Mesa).
    let local_salao = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO locais (id, designacao, mesas_definicao, tipo, nome_generico_mesa, \
         tipo_preco_id) VALUES (?1, 'Salão Principal', '1:99', 'normal', 'Mesa {nm}', ?2)",
    )
    .bind(local_salao.to_string())
    .bind(tp_mesa.to_string())
    .execute(&mut *tx)
    .await?;

    for code in 1..=12 {
        sqlx::query(
            "INSERT INTO tables (id, local_id, code, name, criada_em) VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(local_salao.to_string())
        .bind(code as i64)
        .bind(format!("Mesa {}", code))
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    // Local Take-Away "Balcão" — PVP Take-Away
    let local_balcao = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO locais (id, designacao, tipo, nome_generico_mesa, tipo_preco_id) \
         VALUES (?1, 'Balcão', 'take_away', 'Balcão {nm}', ?2)",
    )
    .bind(local_balcao.to_string())
    .bind(tp_takeaway.to_string())
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "INSERT INTO tables (id, local_id, code, name, criada_em) VALUES (?1, ?2, 1, 'Balcão', ?3)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(local_balcao.to_string())
    .bind(now)
    .execute(&mut *tx)
    .await?;

    // Local Delivery — PVP Delivery
    let local_delivery = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO locais (id, designacao, tipo, nome_generico_mesa, tipo_preco_id) \
         VALUES (?1, 'Delivery', 'delivery', 'Pedido {nm}', ?2)",
    )
    .bind(local_delivery.to_string())
    .bind(tp_delivery.to_string())
    .execute(&mut *tx)
    .await?;

    // Local Consumo Próprio — usa PVP Pessoal (com items grátis/reduzidos)
    let local_consumo = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO locais (id, designacao, tipo, nome_generico_mesa, tipo_preco_id) \
         VALUES (?1, 'Pessoal', 'consumo_proprio', 'Consumo {nm}', ?2)",
    )
    .bind(local_consumo.to_string())
    .bind(tp_pessoal.to_string())
    .execute(&mut *tx)
    .await?;

    // Mappings de impressão para cada local operacional.
    // Cozinha+Bar imprimem em todos os locais que servem comida/bebida.
    for local in [local_salao, local_balcao, local_delivery] {
        for (zona, imp) in [(zi_cozinha, imp_cozinha), (zi_bar, imp_bar)] {
            sqlx::query(
                "INSERT INTO impressora_zona_local (id, zona_impressao_id, local_id, \
                 dispositivo_id, agrupamento, numero_copias) \
                 VALUES (?1, ?2, ?3, ?4, 'normal', 1)",
            )
            .bind(Uuid::new_v4().to_string())
            .bind(zona.to_string())
            .bind(local.to_string())
            .bind(imp.to_string())
            .execute(&mut *tx)
            .await?;
        }
        // Documentos externos imprimem na caixa
        sqlx::query(
            "INSERT INTO impressora_zona_local (id, zona_impressao_id, local_id, \
             dispositivo_id, agrupamento, numero_copias) \
             VALUES (?1, ?2, ?3, ?4, 'normal', 1)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(zi_caixa.to_string())
        .bind(local.to_string())
        .bind(imp_caixa.to_string())
        .execute(&mut *tx)
        .await?;
    }

    // Zonas de entrega
    let zona_centro = Uuid::new_v4();
    let zona_periferia = Uuid::new_v4();
    for (id, codigo, designacao, taxa) in [
        (zona_centro, 1, "Centro", 100_i64),
        (zona_periferia, 2, "Periferia", 250_i64),
    ] {
        sqlx::query(
            "INSERT INTO zonas (id, codigo, designacao, taxa_entrega) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(id.to_string())
        .bind(codigo as i64)
        .bind(designacao)
        .bind(taxa)
        .execute(&mut *tx)
        .await?;
    }

    // Entregadores (motoboys) — separados de empregados.
    for (nome, telefone, externo) in [
        ("Carlos Motoboy", Some("961112233"), true),
        ("João Bike", Some("962224455"), true),
    ] {
        sqlx::query(
            "INSERT INTO entregadores (id, nome, telefone, externo, ativo) \
             VALUES (?1, ?2, ?3, ?4, 1)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(nome)
        .bind(telefone)
        .bind(externo as i32)
        .execute(&mut *tx)
        .await?;
    }

    // Níveis de acesso (spec 1.1) — granular para cancelar/anular.
    let nivel_admin = Uuid::new_v4();
    let nivel_chefe = Uuid::new_v4();
    let nivel_empregado = Uuid::new_v4();
    for (id, codigo, desig, cancela, anula, anula_ci, transf, transf_ci) in [
        (nivel_admin, 1, "Admin", true, true, true, true, true),
        (nivel_chefe, 2, "Chefe de Sala", true, true, false, true, false),
        (nivel_empregado, 3, "Empregado", true, false, false, false, false),
    ] {
        sqlx::query(
            "INSERT INTO niveis_acesso (id, codigo, designacao, cancela_pedidos, \
             anula_pedidos, anula_pedidos_com_conta_impressa, transfere_pedidos, \
             transfere_pedidos_com_conta_impressa) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind(id.to_string())
        .bind(codigo as i64)
        .bind(desig)
        .bind(cancela as i32)
        .bind(anula as i32)
        .bind(anula_ci as i32)
        .bind(transf as i32)
        .bind(transf_ci as i32)
        .execute(&mut *tx)
        .await?;
    }

    // Empregados: Admin paga 100% no consumo, Empregado tem 50% de desconto.
    for (code, name, perc, nivel) in [
        (1, "Admin", 10000_i64, nivel_admin),
        (2, "Empregado", 5000_i64, nivel_empregado),
    ] {
        sqlx::query(
            "INSERT INTO employees (id, code, name, perc_consumo, nivel_acesso_id) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(code as i64)
        .bind(name)
        .bind(perc)
        .bind(nivel.to_string())
        .execute(&mut *tx)
        .await?;
    }

    // Clientes-exemplo, associados a zonas
    for (nome, telefone, morada, zona_id) in [
        ("Cliente Frequente", "915551111", "Rua das Flores, 5", zona_centro),
        ("João Silva", "936669999", "Av. da República, 100", zona_periferia),
    ] {
        sqlx::query(
            "INSERT INTO clientes (id, nome, telefone, morada, localidade, zona_id) \
             VALUES (?1, ?2, ?3, ?4, 'Lisboa', ?5)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(nome)
        .bind(telefone)
        .bind(morada)
        .bind(zona_id.to_string())
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
