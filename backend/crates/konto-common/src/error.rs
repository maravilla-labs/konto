use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, "not_found", m),
            AppError::BadRequest(m) | AppError::Validation(m) => {
                (StatusCode::BAD_REQUEST, "bad_request", m)
            }
            AppError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, "unauthorized", m),
            AppError::Forbidden(m) => (StatusCode::FORBIDDEN, "forbidden", m),
            AppError::Conflict(m) => (StatusCode::CONFLICT, "conflict", m),
            AppError::Internal(m) => {
                tracing::error!("Internal error: {m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "An internal error occurred".to_string())
            }
            AppError::Database(m) => {
                tracing::error!("Database error: {m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "database_error", "An internal error occurred".to_string())
            }
        };

        let body = ErrorResponse {
            error: error_type.to_string(),
            message,
        };

        (status, axum::Json(body)).into_response()
    }
}
