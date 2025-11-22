use axum::response::Json;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "gateway-service",
        "message": "Gateway service is running",
        "routes": {
            "auth": {
                "register": "POST /api/register",
                "login": "POST /api/login",
                "validate": "POST /api/validate"
            },
            "users": {
                "get_all": "GET /api/users",
                "create": "POST /api/users",
                "get_one": "GET /api/users/:id",
                "update": "PUT /api/users/:id",
                "delete": "DELETE /api/users/:id"
            }
        }
    }))
}