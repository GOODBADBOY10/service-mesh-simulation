use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use crate::state::AppState;
    use crate::error::AppError;

/// Forward get all users request to user-service
pub async fn get_users(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let url = format!("{}/users", state.user_service_url);

    let mut request = state.http_client.get(&url);

    // Forward Authorization header
    if let Some(auth) = headers.get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            request = request.header("Authorization", auth_str);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|_| AppError::UserServiceUnavailable)?;

    convert_response(response).await
}

/// Forward create user request to user-service
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Response, AppError> {
    let url = format!("{}/users", state.user_service_url);

    let mut request = state.http_client.post(&url).json(&body);

    if let Some(auth) = headers.get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            request = request.header("Authorization", auth_str);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|_| AppError::UserServiceUnavailable)?;

    convert_response(response).await
}

/// Forward get user by ID request to user-service
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
) -> Result<Response, AppError> {
    let url = format!("{}/users/{}", state.user_service_url, user_id);

    let mut request = state.http_client.get(&url);

    if let Some(auth) = headers.get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            request = request.header("Authorization", auth_str);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|_| AppError::UserServiceUnavailable)?;

    convert_response(response).await
}

/// Forward update user request to user-service
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Response, AppError> {
    let url = format!("{}/users/{}", state.user_service_url, user_id);

    let mut request = state.http_client.put(&url).json(&body);

    if let Some(auth) = headers.get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            request = request.header("Authorization", auth_str);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|_| AppError::UserServiceUnavailable)?;

    convert_response(response).await
}

/// Forward delete user request to user-service
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
) -> Result<Response, AppError> {
    let url = format!("{}/users/{}", state.user_service_url, user_id);

    let mut request = state.http_client.delete(&url);

    if let Some(auth) = headers.get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            request = request.header("Authorization", auth_str);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|_| AppError::UserServiceUnavailable)?;

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


// use std::sync::Arc;
// use axum::{
//     extract::{Path, State},
//     http::{HeaderMap, StatusCode},
//     response::{IntoResponse, Response},
//     Json,
// };
// use serde_json::Value;
// use crate::state::AppState;
// use crate::error::AppError;

// /// Forward get all users request to user-service
// pub async fn get_users(
//     State(state): State<Arc<AppState>>,
//     headers: HeaderMap,
// ) -> Result<Response, AppError> {
//     let url = format!("{}/users", state.user_service_url);

//     let mut request = state.http_client.get(&url);

//     // Forward Authorization header if present
//     if let Some(auth_header) = headers.get("Authorization") {
//         request = request.header("Authorization", auth_header);
//     }

//     let response = request
//         .send()
//         .await
//         .map_err(|_| AppError::UserServiceUnavailable)?;

//     convert_response(response).await
// }

// /// Forward create user request to user-service
// pub async fn create_user(
//     State(state): State<Arc<AppState>>,
//     headers: HeaderMap,
//     Json(body): Json<Value>,
// ) -> Result<Response, AppError> {
//     let url = format!("{}/users", state.user_service_url);

//     let mut request = state.http_client.post(&url).json(&body);

//     // Forward Authorization header if present
//     if let Some(auth_header) = headers.get("Authorization") {
//         request = request.header("Authorization", auth_header);
//     }

//     let response = request
//         .send()
//         .await
//         .map_err(|_| AppError::UserServiceUnavailable)?;

//     convert_response(response).await
// }

// /// Forward get user by ID request to user-service
// pub async fn get_user(
//     State(state): State<Arc<AppState>>,
//     headers: HeaderMap,
//     Path(user_id): Path<String>,
// ) -> Result<Response, AppError> {
//     let url = format!("{}/users/{}", state.user_service_url, user_id);

//     let mut request = state.http_client.get(&url);

//     // Forward Authorization header if present
//     if let Some(auth_header) = headers.get("Authorization") {
//         request = request.header("Authorization", auth_header);
//     }

//     let response = request
//         .send()
//         .await
//         .map_err(|_| AppError::UserServiceUnavailable)?;

//     convert_response(response).await
// }

// /// Forward update user request to user-service
// pub async fn update_user(
//     State(state): State<Arc<AppState>>,
//     headers: HeaderMap,
//     Path(user_id): Path<String>,
//     Json(body): Json<Value>,
// ) -> Result<Response, AppError> {
//     let url = format!("{}/users/{}", state.user_service_url, user_id);

//     let mut request = state.http_client.put(&url).json(&body);

//     // Forward Authorization header if present
//     if let Some(auth_header) = headers.get("Authorization") {
//         request = request.header("Authorization", auth_header);
//     }

//     let response = request
//         .send()
//         .await
//         .map_err(|_| AppError::UserServiceUnavailable)?;

//     convert_response(response).await
// }

// /// Forward delete user request to user-service
// pub async fn delete_user(
//     State(state): State<Arc<AppState>>,
//     headers: HeaderMap,
//     Path(user_id): Path<String>,
// ) -> Result<Response, AppError> {
//     let url = format!("{}/users/{}", state.user_service_url, user_id);

//     let mut request = state.http_client.delete(&url);

//     // Forward Authorization header if present
//     if let Some(auth_header) = headers.get("Authorization") {
//         request = request.header("Authorization", auth_header);
//     }

//     let response = request
//         .send()
//         .await
//         .map_err(|_| AppError::UserServiceUnavailable)?;

//     convert_response(response).await
// }

// /// Convert reqwest Response to axum Response
// async fn convert_response(response: reqwest::Response) -> Result<Response, AppError> {
//     let status = StatusCode::from_u16(response.status().as_u16())
//         .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

//     let body = response
//         .text()
//         .await
//         .map_err(|_| AppError::InternalError)?;

//     Ok((status, body).into_response())
// }