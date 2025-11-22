use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing authorization header")]
    MissingAuthHeader,

    #[error("Invalid authorization header format")]
    InvalidAuthHeader,

    #[error("Invalid or expired token")]
    InvalidToken,

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Auth service unavailable")]
    AuthServiceUnavailable,

    #[error("Forbidden - you can only access your own data")]
    Forbidden,

    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::MissingAuthHeader => (
                StatusCode::UNAUTHORIZED,
                "Missing authorization header",
            ),
            AppError::InvalidAuthHeader => (
                StatusCode::UNAUTHORIZED,
                "Invalid authorization header format",
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token",
            ),
            AppError::UserNotFound => (
                StatusCode::NOT_FOUND,
                "User not found",
            ),
            AppError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "User already exists",
            ),
            AppError::AuthServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Auth service unavailable",
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "Forbidden - you can only access your own data",
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            ),
        };

        let body = Json(ErrorResponse {
            status: status.as_u16().to_string(),
            message: message.to_string(),
        });

        (status, body).into_response()
    }
}