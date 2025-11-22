use axum::http::HeaderMap;
use crate::error::AppError;
use crate::models::{ValidateTokenRequest, ValidateTokenResponse};
use crate::state::AppState;

// Extracts the token from "Authorization: Bearer <token>" header
pub fn extract_token(headers: &HeaderMap) -> Result<String, AppError> {
    // 1. Get the Authorization header
    let auth_header = headers
        .get("Authorization")
        .ok_or(AppError::MissingAuthHeader)?;

    // 2. Convert header to string
    let auth_str = auth_header
        .to_str()
        .map_err(|_| AppError::InvalidAuthHeader)?;

    // 3. Check if it starts with "Bearer "
    if !auth_str.starts_with("Bearer ") {
        return Err(AppError::InvalidAuthHeader);
    }

    // 4. Extract the token part (everything after "Bearer ")
    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(AppError::InvalidAuthHeader)?
        .to_string();

    Ok(token)
}

// Calls auth-service to validate the token
pub async fn validate_token(
    state: &AppState,
    token: &str,
) -> Result<ValidateTokenResponse, AppError> {
    // 1. Build the validate URL
    let url = format!("{}/validate", state.auth_service_url);

    // 2. Create the request body
    let request_body = ValidateTokenRequest {
        token: token.to_string(),
    };

    // 3. Send POST request to auth-service
    let response = state
        .http_client
        .post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|_| AppError::AuthServiceUnavailable)?;

    // 4. Check if auth-service returned success (200)
    if !response.status().is_success() {
        return Err(AppError::InvalidToken);
    }

    // 5. Parse the response body
    let validate_response: ValidateTokenResponse = response
        .json()
        .await
        .map_err(|_| AppError::InternalError)?;

    // 6. Check if token is valid
    if !validate_response.valid {
        return Err(AppError::InvalidToken);
    }

    Ok(validate_response)
}

// Convenience function: extract token AND validate in one call
pub async fn authenticate(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<ValidateTokenResponse, AppError> {
    let token = extract_token(headers)?;
    let user_info = validate_token(state, &token).await?;
    Ok(user_info)
}