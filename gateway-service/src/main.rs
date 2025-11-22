use anyhow::Result;
use crate::route::router;

mod error;
mod state;
mod route;
mod handlers;
mod health_check;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌐 Gateway Service starting...");

    // Internal service URLs
    let auth_service_url = "http://localhost:3000".to_string();
    let user_service_url = "http://localhost:3001".to_string();

    let app = router(auth_service_url.clone(), user_service_url.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;

    println!("✅ Gateway Service running on http://localhost:3002");
    println!();
    println!("🔗 Connected services:");
    println!("   Auth Service: {}", auth_service_url);
    println!("   User Service: {}", user_service_url);
    println!();
    println!("📊 API Routes:");
    println!("   POST   /api/register     → Auth Service");
    println!("   POST   /api/login        → Auth Service");
    println!("   POST   /api/validate     → Auth Service");
    println!("   GET    /api/users        → User Service");
    println!("   POST   /api/users        → User Service");
    println!("   GET    /api/users/:id    → User Service");
    println!("   PUT    /api/users/:id    → User Service");
    println!("   DELETE /api/users/:id    → User Service");
    println!();

    axum::serve(listener, app).await?;

    Ok(())
}
