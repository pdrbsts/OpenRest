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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub id: Uuid,
    pub code: i32,
    pub name: Option<String>,
    pub is_open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    pub id: Uuid,
    pub code: i32,
    pub name: String,
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
