use anyhow::Result;
use std::env;
use crate::route::router;

mod error;
mod state;
mod models;
mod auth;
mod route;
mod handlers;
mod health_check;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧑‍💻 User Service starting...");

    // Read from environment variables with defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let auth_service_url = env::var("AUTH_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let app = router(auth_service_url.clone());

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("✅ User Service running on http://{}", addr);
    println!("🔗 Connected to Auth Service at {}", auth_service_url);

    axum::serve(listener, app).await?;

    Ok(())
}
