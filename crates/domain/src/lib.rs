use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Family {
    pub id: Uuid,
    pub code: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    pub id: Uuid,
    pub family_id: Option<Uuid>,
    pub code: i32,
    pub name: String,
    pub price: i64, // In cents
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
pub struct Document {
    pub id: Uuid,
    pub table_id: Option<Uuid>,
    pub employee_id: Option<Uuid>,
    pub total: i64,
    pub is_closed: bool,
    pub created_at: DateTime<Utc>,
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

impl Article {
    pub fn new(code: i32, name: String, price: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            family_id: None,
            code,
            name,
            price,
            created_at: now,
            updated_at: now,
        }
    }
}
