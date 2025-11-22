use axum::{
    response::{Json},
};
// Health check endpoint
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "API is running",
        "endpoints": {
            "health": "/",
            "register": "/register"
        }
    }))
}