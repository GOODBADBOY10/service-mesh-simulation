use anyhow::Result;
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

    // Auth service URL (in production, use environment variable)
    let auth_service_url = "http://localhost:3000".to_string();

    let app = router(auth_service_url);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;

    println!("✅ User Service running on http://localhost:3001");
    println!("🔗 Connected to Auth Service at http://localhost:3000");
    println!();
    println!("📊 Endpoints:");
    println!("   GET    /           - Health check");
    println!("   GET    /users      - Get all users");
    println!("   POST   /users      - Create user profile");
    println!("   GET    /users/:id  - Get user by ID");
    println!("   PUT    /users/:id  - Update user");
    println!("   DELETE /users/:id  - Delete user");
    println!();

    axum::serve(listener, app).await?;

    Ok(())
}
