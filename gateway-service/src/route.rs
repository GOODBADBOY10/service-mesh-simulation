use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{
    register, login, validate,
    get_users, create_user, get_user, update_user, delete_user,
};
use crate::health_check::health_check;
use crate::state::AppState;

pub fn router(auth_service_url: String, user_service_url: String) -> Router {
    let state = AppState::new(auth_service_url, user_service_url);

    Router::new()
        // Health check
        .route("/", get(health_check))
        
        // Auth routes (forwarded to auth-service)
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/validate", post(validate))
        
        // User routes (forwarded to user-service)
        .route("/api/users", get(get_users))
        .route("/api/users", post(create_user))
        .route("/api/users/{id}", get(get_user))
        .route("/api/users/{id}", put(update_user))
        .route("/api/users/{id}", delete(delete_user))
        
        .with_state(Arc::new(state))
}