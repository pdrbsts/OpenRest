use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Spec §57 "Data Lógica de Caixa": calcula o Dia de facturação a partir do
/// instante do relógio (em hora local da loja) e do ponto de corte.
///
/// `cutoff_minutes` é o nº de minutos desde a meia-noite a partir do qual já
/// estamos num novo Dia (ex: 02:00 → 120). Por defeito 0 (dia civil normal).
///
/// Antes do corte → o documento entra no Dia anterior.
pub fn compute_business_day(now_local: NaiveDateTime, cutoff_minutes: u32) -> NaiveDate {
    let minutes_today = now_local.time().num_seconds_from_midnight() / 60;
    if minutes_today < cutoff_minutes {
        now_local.date() - Duration::days(1)
    } else {
        now_local.date()
    }
}

/// Converte `HH:MM` para nº de minutos desde a meia-noite. Retorna `None` se
/// o formato for inválido ou os valores fora de gama.
pub fn parse_cutoff_hhmm(s: &str) -> Option<u32> {
    let (h, m) = s.split_once(':')?;
    let h: u32 = h.parse().ok()?;
    let m: u32 = m.parse().ok()?;
    if h >= 24 || m >= 60 {
        return None;
    }
    Some(h * 60 + m)
}

/// Converte um instante UTC para hora local aplicando um offset em minutos.
pub fn utc_to_local(now_utc: DateTime<Utc>, tz_offset_minutes: i32) -> NaiveDateTime {
    (now_utc + Duration::minutes(tz_offset_minutes as i64)).naive_utc()
}

#[cfg(test)]
mod business_day_tests {
    use super::*;
    use chrono::NaiveDate;

    fn dt(y: i32, m: u32, d: u32, h: u32, min: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, min, 0)
            .unwrap()
    }

    #[test]
    fn cafe_cutoff_02_00() {
        let cutoff = parse_cutoff_hhmm("02:00").unwrap();
        // 23:00 do dia 27 → Dia = 27 (já passou as 02:00 da manhã desse dia)
        assert_eq!(
            compute_business_day(dt(2026, 5, 27, 23, 0), cutoff),
            NaiveDate::from_ymd_opt(2026, 5, 27).unwrap()
        );
        // 00:30 do dia 28 → Dia = 27 (ainda não chegou ao corte das 02:00)
        assert_eq!(
            compute_business_day(dt(2026, 5, 28, 0, 30), cutoff),
            NaiveDate::from_ymd_opt(2026, 5, 27).unwrap()
        );
        // 01:59 do dia 28 → Dia = 27 (fronteira inclusiva)
        assert_eq!(
            compute_business_day(dt(2026, 5, 28, 1, 59), cutoff),
            NaiveDate::from_ymd_opt(2026, 5, 27).unwrap()
        );
        // 02:00 exactas do dia 28 → Dia = 28 (corte é >=)
        assert_eq!(
            compute_business_day(dt(2026, 5, 28, 2, 0), cutoff),
            NaiveDate::from_ymd_opt(2026, 5, 28).unwrap()
        );
        // 02:15 do dia 28 → Dia = 28
        assert_eq!(
            compute_business_day(dt(2026, 5, 28, 2, 15), cutoff),
            NaiveDate::from_ymd_opt(2026, 5, 28).unwrap()
        );
        // 08:00 do dia 28 → Dia = 28
        assert_eq!(
            compute_business_day(dt(2026, 5, 28, 8, 0), cutoff),
            NaiveDate::from_ymd_opt(2026, 5, 28).unwrap()
        );
    }

    #[test]
    fn cutoff_zero_is_civil_day() {
        // Sem corte → dia civil normal.
        assert_eq!(
            compute_business_day(dt(2026, 5, 28, 0, 0), 0),
            NaiveDate::from_ymd_opt(2026, 5, 28).unwrap()
        );
        assert_eq!(
            compute_business_day(dt(2026, 5, 28, 23, 59), 0),
            NaiveDate::from_ymd_opt(2026, 5, 28).unwrap()
        );
    }

    #[test]
    fn month_and_year_rollback() {
        let cutoff = parse_cutoff_hhmm("03:00").unwrap();
        // 01 Jan às 01:00 → Dia = 31 Dez do ano anterior.
        assert_eq!(
            compute_business_day(dt(2027, 1, 1, 1, 0), cutoff),
            NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()
        );
        // 01 Jun às 02:30 → Dia = 31 Mai.
        assert_eq!(
            compute_business_day(dt(2026, 6, 1, 2, 30), cutoff),
            NaiveDate::from_ymd_opt(2026, 5, 31).unwrap()
        );
    }

    #[test]
    fn parse_cutoff_strings() {
        assert_eq!(parse_cutoff_hhmm("00:00"), Some(0));
        assert_eq!(parse_cutoff_hhmm("02:00"), Some(120));
        assert_eq!(parse_cutoff_hhmm("23:59"), Some(23 * 60 + 59));
        assert_eq!(parse_cutoff_hhmm("24:00"), None);
        assert_eq!(parse_cutoff_hhmm("12:60"), None);
        assert_eq!(parse_cutoff_hhmm("abc"), None);
        assert_eq!(parse_cutoff_hhmm(""), None);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Family {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub code: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    pub id: Uuid,
    pub family_id: Option<Uuid>,
    pub code: i32,
    pub name: String,
    /// 5 tabelas de preço (PVP1..PVP5), em cêntimos. Cada local escolhe via tipo_preco.
    /// pvp2..pvp5 são opcionais: `None` significa "não configurado, usa pvp1";
    /// `Some(0)` significa "grátis".
    pub pvp1: i64,
    pub pvp2: Option<i64>,
    pub pvp3: Option<i64>,
    pub pvp4: Option<i64>,
    pub pvp5: Option<i64>,
    /// IVA em basis points (1300 = 13%).
    pub vat_rate: i32,
    /// normal | complemento | informativo | consumo | gorjeta
    pub tipo_artigo: String,
    pub zona_impressao_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Article {
    /// Devolve o PVP correspondente ao código de tabela (1..5).
    /// Quando o PVP escolhido é `None`, faz fallback para pvp1; `Some(0)` significa grátis.
    pub fn pvp_for(&self, codigo: i32) -> i64 {
        match codigo {
            2 => self.pvp2.unwrap_or(self.pvp1),
            3 => self.pvp3.unwrap_or(self.pvp1),
            4 => self.pvp4.unwrap_or(self.pvp1),
            5 => self.pvp5.unwrap_or(self.pvp1),
            _ => self.pvp1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TipoPreco {
    pub id: Uuid,
    pub codigo: i32,
    pub designacao: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Zona {
    pub id: Uuid,
    pub codigo: Option<i32>,
    pub designacao: String,
    pub taxa_entrega: i64,
    pub rede_remota_associada_id: Option<Uuid>,
    pub anulado_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entregador {
    pub id: Uuid,
    pub nome: String,
    pub telefone: Option<String>,
    pub externo: bool,
    pub ativo: bool,
    pub anulado_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dispositivo {
    pub id: Uuid,
    pub nome: String,
    pub tipo: String,
    pub modelo: Option<String>,
    pub descricao: Option<String>,
    pub output_path: Option<String>,
    pub ativo: bool,
    pub anulado_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZonaImpressao {
    pub id: Uuid,
    pub codigo: i32,
    pub designacao: String,
    pub secundarios: bool,
    pub anulado_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImpressoraZonaLocal {
    pub id: Uuid,
    pub zona_impressao_id: Uuid,
    pub local_id: Uuid,
    pub origem_id: Option<Uuid>,
    pub dispositivo_id: Uuid,
    pub agrupamento: String,
    pub numero_copias: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum LocalKind {
    Normal,
    TakeAway,
    TakeAwaySeguro,
    Pub,
    Delivery,
    ConsumoProprio,
    RestauracaoColectiva,
}

impl LocalKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            LocalKind::Normal => "normal",
            LocalKind::TakeAway => "take_away",
            LocalKind::TakeAwaySeguro => "take_away_seguro",
            LocalKind::Pub => "pub",
            LocalKind::Delivery => "delivery",
            LocalKind::ConsumoProprio => "consumo_proprio",
            LocalKind::RestauracaoColectiva => "restauracao_colectiva",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "normal" => LocalKind::Normal,
            "take_away" => LocalKind::TakeAway,
            "take_away_seguro" => LocalKind::TakeAwaySeguro,
            "pub" => LocalKind::Pub,
            "delivery" => LocalKind::Delivery,
            "consumo_proprio" => LocalKind::ConsumoProprio,
            "restauracao_colectiva" => LocalKind::RestauracaoColectiva,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Local {
    pub id: Uuid,
    pub designacao: String,
    pub mesas_definicao: Option<String>,
    pub tipo: LocalKind,
    pub tipo_preco_id: Option<Uuid>,
    pub metodo_pagamento_default_id: Option<Uuid>,
    pub taxa_servico_artigo_id: Option<Uuid>,
    pub limite_consumo: i64,
    pub imprime_conta_acima_de: i64,
    pub nome_generico_mesa: String,
    pub imprime_subtotal_em: serde_json::Value,
    pub imprime_conta_em: serde_json::Value,
    pub fecha_mesa_ao_pedir: String, // nunca|comando|sempre
    pub usa_iva_venda_directa: bool,
    pub iva_excluido_dos_precos: bool,
    pub cor_empregado_na_lista: bool,
    pub impressora_directa_pedidos_id: Option<Uuid>,
    pub pede_nova_mesa_depois_de_fechar: bool,
    pub pede_nova_mesa_apos_pedido: bool,
    pub indica_pessoas_obrigatorio: bool,
    pub indica_pessoas_apenas_abertura: bool,
    pub permite_zero_pessoas: bool,
    pub aloca_mesas_dinamicamente: bool,
    pub alocacao_circular: bool,
    pub inclui_desconto_nos_precos: bool,
    pub artigos_automatico_sem_preco: bool,
    pub carregamento_rapido_mesas: bool,
    pub so_imprime_pedidos_com_complementos: bool,
    pub lista_grande_pedidos: bool,
    pub mesas_uma_vez_por_dia: bool,
    pub facturacao_externa: bool,
    pub nao_agrupa_detalhes_na_conta: bool,
    pub permite_encaixe_promocoes: bool,
    pub separa_artigos_antes_encaixe: bool,
    pub permite_mesas_abertas_fim_do_dia: bool,
    pub pode_identificar_cliente_no_pedido: bool,
    pub obriga_indicar_valor_pago: bool,
    pub usa_desenho_mesas: bool,
    pub imagem: Option<String>,
    pub largura: Option<i32>,
    pub altura: Option<i32>,
    pub anulado_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub id: Uuid,
    pub local_id: Option<Uuid>,
    pub code: i32,
    pub name: Option<String>,
    pub nomeobjecto: Option<String>,
    pub posx: Option<i32>,
    pub posy: Option<i32>,
    pub imagem: Option<String>,
    pub fntname: Option<String>,
    pub fntsize: Option<i32>,
    pub fntcolor: Option<String>,
    pub fontx: Option<i32>,
    pub fonty: Option<i32>,
    pub fontstyle: Option<String>,
    pub estadox: Option<i32>,
    pub estadoy: Option<i32>,
    pub reservax: Option<i32>,
    pub reservay: Option<i32>,
    pub altura: Option<i32>,
    pub largura: Option<i32>,
    pub criada_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MesaEstadoKind {
    Livre,
    Aberta,
    EmEspera,
    Reservada,
    Bloqueada,
}

impl MesaEstadoKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            MesaEstadoKind::Livre => "livre",
            MesaEstadoKind::Aberta => "aberta",
            MesaEstadoKind::EmEspera => "em_espera",
            MesaEstadoKind::Reservada => "reservada",
            MesaEstadoKind::Bloqueada => "bloqueada",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "livre" => MesaEstadoKind::Livre,
            "aberta" => MesaEstadoKind::Aberta,
            "em_espera" => MesaEstadoKind::EmEspera,
            "reservada" => MesaEstadoKind::Reservada,
            "bloqueada" => MesaEstadoKind::Bloqueada,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MesaEstado {
    pub mesa_id: Uuid,
    pub estado: MesaEstadoKind,
    pub bloqueada_por_posto_id: Option<Uuid>,
    pub bloqueada_motivo: Option<String>,
    pub cliente_associado_id: Option<Uuid>,
    pub numero_pessoas: Option<i32>,
    pub empregado_actual_id: Option<Uuid>,
    pub aberta_em: Option<DateTime<Utc>>,
    pub subtotal_actual: i64,
    pub reservada_ate: Option<DateTime<Utc>>,
    pub reserva_pessoas: Option<i32>,
    pub reserva_cliente_id: Option<Uuid>,
    pub reserva_observacoes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    pub id: Uuid,
    pub code: i32,
    pub name: String,
    /// Percentagem que o empregado paga sobre o PVP em consumo próprio
    /// (basis points: 10000 = 100%, 5000 = 50%).
    pub perc_consumo: i32,
    pub base_consumo: i64,
    pub nivel_acesso_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NivelAcesso {
    pub id: Uuid,
    pub codigo: i32,
    pub designacao: String,
    pub cancela_pedidos: bool,
    pub anula_pedidos: bool,
    pub anula_pedidos_com_conta_impressa: bool,
    pub transfere_pedidos: bool,
    pub transfere_pedidos_com_conta_impressa: bool,
    pub anulado_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transferencia {
    pub id: Uuid,
    pub from_document_id: Uuid,
    pub to_document_id: Uuid,
    pub line_id: Uuid,
    pub article_id: Uuid,
    pub qty: i64,
    pub employee_id: Option<Uuid>,
    pub transferida_em: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Customer {
    pub id: Uuid,
    pub codigo: Option<i32>,
    pub nome: String,
    pub nif: Option<String>,
    pub pais: String,
    pub telefone: Option<String>,
    pub morada: Option<String>,
    pub cod_postal: Option<String>,
    pub localidade: Option<String>,
    pub email: Option<String>,
    pub observacoes: Option<String>,
    pub numero_cartao: Option<String>,
    pub limite_credito: i64,
    pub zona_id: Option<Uuid>,
    pub anulado_em: Option<DateTime<Utc>>,
    pub esquecido_em: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryEstado {
    Recebido,
    EmPreparacao,
    Pronto,
    Despachado,
    Entregue,
    Cancelado,
}

impl DeliveryEstado {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeliveryEstado::Recebido => "recebido",
            DeliveryEstado::EmPreparacao => "em_preparacao",
            DeliveryEstado::Pronto => "pronto",
            DeliveryEstado::Despachado => "despachado",
            DeliveryEstado::Entregue => "entregue",
            DeliveryEstado::Cancelado => "cancelado",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "recebido" => DeliveryEstado::Recebido,
            "em_preparacao" => DeliveryEstado::EmPreparacao,
            "pronto" => DeliveryEstado::Pronto,
            "despachado" => DeliveryEstado::Despachado,
            "entregue" => DeliveryEstado::Entregue,
            "cancelado" => DeliveryEstado::Cancelado,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PedidoDelivery {
    pub id: Uuid,
    pub document_id: Uuid,
    pub cliente_id: Option<Uuid>,
    pub morada_snapshot: Option<String>,
    pub telefone_snapshot: Option<String>,
    pub recebido_em: DateTime<Utc>,
    pub recebido_via: String,
    pub entregador_id: Option<Uuid>,
    pub pronto_em: Option<DateTime<Utc>>,
    pub despachado_em: Option<DateTime<Utc>>,
    pub entregue_em: Option<DateTime<Utc>>,
    pub estado: DeliveryEstado,
    pub zona_id: Option<Uuid>,
    pub taxa_entrega_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentMethod {
    pub id: Uuid,
    pub code: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentSeries {
    pub id: Uuid,
    pub document_type: String,
    pub prefix: String,
    pub year: i32,
    pub next_number: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Atcud {
    pub id: Uuid,
    pub document_type: String,
    pub series_prefix: String,
    pub year: i32,
    pub atcud: String,
    pub start_date: chrono::NaiveDate,
    pub registered_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub id: Uuid,
    pub table_id: Option<Uuid>,
    pub employee_id: Option<Uuid>,
    pub total: i64,
    pub is_closed: bool,
    pub created_at: DateTime<Utc>,

    pub series_id: Option<Uuid>,
    pub document_type: Option<String>,
    pub document_number: Option<i32>,
    pub atcud: Option<String>,
    pub hash: Option<String>,
    pub hash_short: Option<String>,
    pub previous_hash: Option<String>,
    pub issued_at: Option<DateTime<Utc>>,
    pub qr_payload: Option<String>,

    pub customer_id: Option<Uuid>,
    pub local_id: Option<Uuid>,
    pub observacoes_pedido: Option<String>,
    pub observacoes_factura: Option<String>,
    pub observacoes_cliente: Option<String>,
    pub observacoes_morada: Option<String>,
    pub delivery_morada: Option<String>,
    pub delivery_telefone: Option<String>,
    pub subtotal_impresso_em: Option<DateTime<Utc>>,
    pub data_dia: Option<NaiveDate>,
    pub sessao_id: Option<Uuid>,
    pub troco_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessaoEmpregado {
    pub id: Uuid,
    pub empregado_id: Uuid,
    pub data_dia: NaiveDate,
    pub com_bolsa: bool,
    pub fundo_bolsa: i64,
    pub observacao_abertura: Option<String>,
    pub observacao_fecho: Option<String>,
    pub aberta_em: DateTime<Utc>,
    pub aberta_por: Option<Uuid>,
    pub fechada_em: Option<DateTime<Utc>>,
    pub fechada_por: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentDetail {
    pub id: Uuid,
    pub document_id: Uuid,
    pub article_id: Uuid,
    pub qty: i32,
    pub unit_price: i64,
    pub total: i64,
    pub pedida_em: Option<DateTime<Utc>>,
    pub anulada: bool,
    pub anulada_com_desperdicio: bool,
    pub anulada_em: Option<DateTime<Utc>>,
    pub anulada_por: Option<Uuid>,
    pub anulada_motivo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Anulacao {
    pub id: Uuid,
    pub document_id: Uuid,
    pub document_detail_id: Uuid,
    pub article_id: Uuid,
    pub qty: i32,
    pub unit_price: i64,
    pub total: i64,
    pub com_desperdicio: bool,
    pub motivo: Option<String>,
    pub empregado_id: Option<Uuid>,
    pub anulada_em: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cancelamento {
    pub id: Uuid,
    pub document_id: Uuid,
    pub article_id: Uuid,
    pub qty: i32,
    pub unit_price: i64,
    pub total: i64,
    pub motivo: Option<String>,
    pub empregado_id: Option<Uuid>,
    pub cancelada_em: DateTime<Utc>,
}

/// Rodapé de pagamento de um documento. Um documento pode ter N rodapés
/// (múltiplos métodos de pagamento). O `amount` é em cêntimos (i64) e o
/// somatório dos pagamentos cobre o total do documento; quando soma > total,
/// o excedente é registado em `Document.troco_cents` e o último método
/// (normalmente numerário) absorve a diferença.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    pub id: Uuid,
    pub document_id: Uuid,
    pub payment_method_id: Uuid,
    pub amount: i64,
    pub descricao: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Article {
    pub fn new(code: i32, name: String, price: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            family_id: None,
            code,
            name,
            pvp1: price,
            pvp2: None,
            pvp3: None,
            pvp4: None,
            pvp5: None,
            vat_rate: 1300,
            tipo_artigo: "normal".into(),
            zona_impressao_id: None,
            created_at: now,
            updated_at: now,
        }
    }
}
