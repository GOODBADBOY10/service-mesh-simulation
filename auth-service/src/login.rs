use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use axum::{
    extract::State,
    response::Json,
};
use crate::{error::AppError, state::AppState};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: i64,
}

pub const JWT_SECRET: &[u8] = b"your-secret-key-change-this-in-production";

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
    token_type: String,
    expires_in: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}


pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    
    // 1. Lock the HashMap
    let users = state.users.lock()
        .map_err(|_| AppError::InternalError)?;
    
    // 2. Find the user by username
    let user = users.get(&req.username)
        .ok_or(AppError::InvalidCredentials)?;
    
    // 3. Verify password
    let password_valid = verify(&req.password, &user.password_hash)
        .map_err(|_| AppError::InternalError)?;
    
    if !password_valid {
        return Err(AppError::InvalidCredentials);
    }
    
    // 4. Generate JWT token
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .ok_or(AppError::InternalError)?  // ✅ Fixed!
        .timestamp();
    
    let claims = Claims {
        sub: user.user_id.clone(),
        username: user.username.clone(),
        exp: expiration,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET)
    ).map_err(|_| AppError::TokenGenerationError)?;
    
    // 5. Return token
    Ok(Json(LoginResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: 86400,
    }))
}