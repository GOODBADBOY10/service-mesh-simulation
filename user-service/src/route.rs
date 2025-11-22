use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{get_user, get_users, create_user, update_user, delete_user};
use crate::health_check::health_check;
use crate::state::AppState;

pub fn router(auth_service_url: String) -> Router {
    let state = AppState::new(auth_service_url);

    Router::new()
        .route("/", get(health_check))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(Arc::new(state))
}