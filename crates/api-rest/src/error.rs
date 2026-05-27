use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use storage::StorageError;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    BadRequest(String),
    #[error("storage: {0}")]
    Storage(#[from] StorageError),
    #[error("internal: {0}")]
    Internal(String),
}

impl From<storage::sqlx::Error> for ApiError {
    fn from(e: storage::sqlx::Error) -> Self {
        ApiError::Storage(StorageError::Database(e))
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match &self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::Storage(StorageError::NotFound) => {
                (StatusCode::NOT_FOUND, "not found".to_string())
            }
            ApiError::Storage(e) => {
                tracing::error!("storage error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "storage error".into())
            }
            ApiError::Internal(_) => {
                tracing::error!("internal error: {self}");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };
        (status, Json(ErrorBody { error: msg })).into_response()
    }
}
