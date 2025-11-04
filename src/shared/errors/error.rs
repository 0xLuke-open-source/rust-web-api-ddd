use crate::shared::api_response::ApiResponse;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Internal server error")]
    Internal,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Hashing error: {0}")]
    Hashing(#[from] argon2::password_hash::Error),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, _, code) = match self {
            Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error", 500),
            Error::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error", 400),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, "Not found", 404),
            Error::Conflict(_) => (StatusCode::CONFLICT, "Conflict", 409),
            Error::Auth(_) => (StatusCode::UNAUTHORIZED, "Authentication error", 401),
            Error::Hashing(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Hashing error", 500),
            Error::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                500,
            ),
            Error::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "Unauthorized", 401),
            Error::InvalidRequest(_) => (StatusCode::BAD_REQUEST, "Invalid request", 400),
            Error::InvalidToken(_) => (StatusCode::UNAUTHORIZED, "Invalid token", 401),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                500,
            ),
        };
        let message = self.to_string();
        let response = ApiResponse::<()>::error_with_code(code, &message);
        (status, Json(response)).into_response()
    }
}
