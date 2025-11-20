use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::get,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(Debug, Deserialize, Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RegisterResponse {
    user_id: String,
    username: String,
    message: String,
}
 
#[derive(Debug, Serialize)] 
struct ErrorResponse {
    status: String,
    message: String,
}

struct User {
    user_id: String,
    username: String,
    email: String,
    password_hash: String,
}
struct AppState {
    users: Mutex<HashMap<String, User>>,
}

#[tokio::main]
async fn main() {
    println!("🚀 API Server starting...");
    
    let state = AppState {
        users: Mutex::new(HashMap::new()),
    };

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/register", post(register))
        .with_state(Arc::new(state));

    // Bind to port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("✅ Server running on http://localhost:3000");
    println!("📊 Health check: http://localhost:3000/");
    println!();

    // Start serving
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "API is running",
        "endpoints": {
            "health": "/",
            "register": "/register"
        }
    }))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
     todo!("Implement registration logic")
}