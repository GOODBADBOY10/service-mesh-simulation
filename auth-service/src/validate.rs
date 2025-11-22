use axum::response::Json;
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use crate::login::{Claims, JWT_SECRET};  // Import from your login module


#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct ValidateResponse {
    valid: bool,
    user_id: String,
    username: String,
}


pub async fn validate(
    Json(req): Json<ValidateRequest>,
) -> Result<Json<ValidateResponse>, AppError> {
    
    // 1. Decode and verify the token
    let token_data = decode::<Claims>(
        &req.token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default()
    ).map_err(|_| AppError::InvalidToken)?;
    
    // 2. Extract claims (user info)
    let claims = token_data.claims;
    
    // 3. Check if token is expired
    if claims.exp < Utc::now().timestamp() {
        return Err(AppError::TokenExpired);
    }
    
    // 4. Return user info
    Ok(Json(ValidateResponse {
        valid: true,
        user_id: claims.sub,
        username: claims.username,
    }))
}