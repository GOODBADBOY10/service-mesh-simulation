use std::sync::Arc;
use axum::{
    extract::State,
    response::{Json},
};
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::AppState};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    user_id: String,
    username: String,
    message: String,
}

pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}


pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> { 
    // 1. Lock the HashMap - handle lock error properly
    let mut users = state.users.lock()
        .map_err(|_| AppError::InternalError)?;
    
    // 2. Check if username already exists
    if users.contains_key(&req.username) {
        return Err(AppError::UserAlreadyExists);
    }

    // 3. Check if email already exists
    for user in users.values() {
        if user.email == req.email {
            return Err(AppError::EmailAlreadyExists);
        }
    }
    
    // 4. Hash the password - handle error properly
    let password_hash = hash(req.password, DEFAULT_COST)
        .map_err(|_| AppError::PasswordHashError)?;
    
    // 5. Generate user ID
    let user_id = Uuid::new_v4().to_string();
    
    // 6. Create user
    let user = User {
        user_id: user_id.clone(),
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash,
    };
    
    // 7. Store in HashMap
    users.insert(req.username.clone(), user);
    
    // 8. Create response
    let response = RegisterResponse {
        user_id,
        username: req.username,
        message: "User registered successfully".to_string(),
    };
    
    // 9. Return success
    Ok(Json(response))
}