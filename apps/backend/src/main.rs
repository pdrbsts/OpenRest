use std::sync::Arc;
use server::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // Initialize shared state
    let state = Arc::new(AppState::new());
    
    // Create REST API router
    let app = api_rest::create_router(state);
    
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Backend server listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}
