use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub price: i64,
    /// IVA em basis points (1300 = 13%).
    pub vat_rate: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Customer {
    pub id: Uuid,
    pub codigo: Option<i32>,
    pub nome: String,
    pub nif: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentDetail {
    pub id: Uuid,
    pub document_id: Uuid,
    pub article_id: Uuid,
    pub qty: i32,
    pub unit_price: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    pub id: Uuid,
    pub document_id: Uuid,
    pub payment_method_id: Uuid,
    pub amount: i64,
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
            price,
            vat_rate: 1300,
            created_at: now,
            updated_at: now,
        }
    }
}
