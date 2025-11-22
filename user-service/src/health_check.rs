use axum::response::Json;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "user-service",
        "message": "User service is running",
        "endpoints": {
            "health": "GET /",
            "get_user": "GET /users/:id",
            "get_users": "GET /users",
            "create_user": "POST /users",
            "update_user": "PUT /users/:id",
            "delete_user": "DELETE /users/:id"
        }
    }))
}