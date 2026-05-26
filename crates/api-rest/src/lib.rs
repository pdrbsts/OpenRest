use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use server::AppState;
use domain::{Article, Family, Table};
use uuid::Uuid;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/catalog", get(get_catalog))
        .route("/api/tables", get(get_tables))
        .route("/api/orders", post(create_order))
        .with_state(state)
}

#[derive(Serialize)]
pub struct CatalogResponse {
    pub families: Vec<Family>,
    pub articles: Vec<Article>,
}

async fn get_catalog(State(_state): State<Arc<AppState>>) -> Json<CatalogResponse> {
    // TODO: Query from storage. Returning mock data for now to validate setup.
    let mock_family = Family {
        id: Uuid::new_v4(),
        code: 100,
        name: "Cafetaria".to_string(),
    };
    
    let mut mock_article = Article::new(1, "Café Expresso".to_string(), 80);
    mock_article.family_id = Some(mock_family.id);

    Json(CatalogResponse {
        families: vec![mock_family],
        articles: vec![mock_article],
    })
}

async fn get_tables(State(_state): State<Arc<AppState>>) -> Json<Vec<Table>> {
    let mock_table = Table {
        id: Uuid::new_v4(),
        code: 1,
        name: Some("Mesa 1".to_string()),
        is_open: false,
    };
    
    Json(vec![mock_table])
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub table_id: Uuid,
    pub employee_id: Uuid,
    pub lines: Vec<OrderLine>,
}

#[derive(Deserialize)]
pub struct OrderLine {
    pub article_id: Uuid,
    pub qty: i32,
    pub unit_price: i64,
}

#[derive(Serialize)]
pub struct CreateOrderResponse {
    pub document_id: Uuid,
    pub total: i64,
}

async fn create_order(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrderRequest>,
) -> Json<CreateOrderResponse> {
    // In Phase 1 we calculate the total and generate a mock order
    let total: i64 = payload.lines.iter().map(|l| l.unit_price * l.qty as i64).sum();
    
    Json(CreateOrderResponse {
        document_id: Uuid::new_v4(),
        total,
    })
}
