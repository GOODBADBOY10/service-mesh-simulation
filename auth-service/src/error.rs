use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    status: String,
    message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Username already exists")]
    UserAlreadyExists,

    #[error("Email already registered")]
    EmailAlreadyExists,

    #[error("Failed to hash password")]
    PasswordHashError,

    #[error("Internal server error")]
    InternalError,

    #[error("Invalid username or password")]  // ← Fixed error message
    InvalidCredentials,
    
    #[error("Failed to generate token")]
    TokenGenerationError,
    
    #[error("Invalid or malformed token")]
    InvalidToken,
    
    #[error("Token has expired")]
    TokenExpired,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            // ✅ Original cases
            AppError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "Username already exists",
            ),
            AppError::EmailAlreadyExists => (
                StatusCode::CONFLICT,
                "Email already registered",
            ),
            AppError::PasswordHashError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to hash password",
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            ),
            // ✅ New cases
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid username or password",
            ),
            AppError::TokenGenerationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate token",
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid or malformed token",
            ),
            AppError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "Token has expired",
            ),
        };

        let body = Json(ErrorResponse {
            status: status.as_u16().to_string(),
            message: error_message.to_string(),
        });

        (status, body).into_response()
    }
}






// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         let status = match self {
//             AppError::UserAlreadyExists | AppError::EmailAlreadyExists => StatusCode::CONFLICT,
//             AppError::InvalidCredentials | AppError::InvalidToken | AppError::TokenExpired => StatusCode::UNAUTHORIZED,
//             _ => StatusCode::INTERNAL_SERVER_ERROR,
//         };

//         let body = Json(ErrorResponse {
//             status: status.as_u16().to_string(),
//             message: self.to_string(),  // ✅ Uses the #[error] message automatically!
//         });

//         (status, body).into_response()
//     }
// }