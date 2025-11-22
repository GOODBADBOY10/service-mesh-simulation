use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use crate::state::AppState;
use crate::error::AppError;

/// Forward registration request to auth-service
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> Result<Response, AppError> {
    let url = format!("{}/register", state.auth_service_url);

    let response = state
        .http_client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|_| AppError::AuthServiceUnavailable)?;

    convert_response(response).await
}

/// Forward login request to auth-service
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> Result<Response, AppError> {
    let url = format!("{}/login", state.auth_service_url);

    let response = state
        .http_client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|_| AppError::AuthServiceUnavailable)?;

    convert_response(response).await
}

/// Forward validate request to auth-service
pub async fn validate(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> Result<Response, AppError> {
    let url = format!("{}/validate", state.auth_service_url);

    let response = state
        .http_client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|_| AppError::AuthServiceUnavailable)?;

    convert_response(response).await
}

/// Convert reqwest Response to axum Response
async fn convert_response(response: reqwest::Response) -> Result<Response, AppError> {
    let status = StatusCode::from_u16(response.status().as_u16())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let body = response
        .text()
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok((status, body).into_response())
}