use anyhow::Result;
use std::env;
use crate::route::router;

mod error;
mod state;
mod route;
mod health_check;
mod register;
mod login;
mod validate;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 Auth Service starting...");

    // Read from environment variables with defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let app = router();

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("✅ Auth Service running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}