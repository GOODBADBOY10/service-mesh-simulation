use std::{collections::HashMap, sync::{Arc, Mutex}};
use axum::{routing::get, routing::post, Router };
use crate::{health_check::health_check, login::login, validate::validate};
use crate::register::register;
use crate::{state::AppState};

pub fn router() -> Router {
    let state = AppState {
        users: Mutex::new(HashMap::new()),
    };
    
    Router::new()
        .route("/", get(health_check))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/validate", post(validate))
        .with_state(Arc::new(state))
}