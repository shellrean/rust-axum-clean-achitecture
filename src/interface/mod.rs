pub mod routes;
pub mod handlers;
pub mod state;
pub mod layers;

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Database(msg) => {
                tracing::error!("database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("99"),
                    String::from("internal server database issue"),
                )
            }
            AppError::Authentication(msg) => {
                tracing::error!("authentication error: {}", msg);
                (StatusCode::UNAUTHORIZED, String::from("authentication error"), msg)
            }
        };

        let body = ErrorDetail { code, message };

        (status, Json(body)).into_response()
    }
}
