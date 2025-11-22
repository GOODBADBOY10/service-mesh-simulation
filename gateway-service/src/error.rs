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
    #[error("Auth service unavailable")]
    AuthServiceUnavailable,

    #[error("User service unavailable")]
    UserServiceUnavailable,

    // #[error("Bad request")]
    // BadRequest,

    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::AuthServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Auth service unavailable",
            ),
            AppError::UserServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "User service unavailable",
            ),
            // AppError::BadRequest => (
                // StatusCode::BAD_REQUEST,
                // "Bad request",
            // ),
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