use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(String),
    #[error("authentication error: {0}")]
    Authentication(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("99"), String::from("internal server database issue")),
            AppError::Authentication(msg) => {
                tracing::error!("authentication error: {}", msg);
                (StatusCode::UNAUTHORIZED, String::from("authentication error"), msg)
            }
        };

        let body = ErrorDetail {
            code,
            message
        };

        (status, Json(body)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: Error) -> Self {
        match value{
            _ => AppError::Database(value.to_string()),
        }
    }
}