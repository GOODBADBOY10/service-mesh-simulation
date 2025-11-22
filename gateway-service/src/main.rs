use anyhow::Result;
use std::env;
use crate::route::router;

mod error;
mod state;
mod route;
mod handlers;
mod health_check;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌐 Gateway Service starting...");

    // Read from environment variables with defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3002".to_string());
    let auth_service_url = env::var("AUTH_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    let user_service_url = env::var("USER_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3001".to_string());

    let app = router(auth_service_url.clone(), user_service_url.clone());

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("✅ Gateway Service running on http://{}", addr);
    println!("🔗 Auth Service: {}", auth_service_url);
    println!("🔗 User Service: {}", user_service_url);

    axum::serve(listener, app).await?;

    Ok(())
}

//  println!("📊 API Routes:");
//     println!("   POST   /api/register     → Auth Service");
//     println!("   POST   /api/login        → Auth Service");
//     println!("   POST   /api/validate     → Auth Service");
//     println!("   GET    /api/users        → User Service");
//     println!("   POST   /api/users        → User Service");
//     println!("   GET    /api/users/:id    → User Service");
//     println!("   PUT    /api/users/:id    → User Service");
//     println!("   DELETE /api/users/:id    → User Service");
//     println!();