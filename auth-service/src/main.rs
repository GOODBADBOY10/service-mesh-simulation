use anyhow::Result;
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
    println!("🚀 API Server starting...");
    
    let app = router();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    
    println!("✅ Server running on http://localhost:3000");
    println!("📊 Health check: http://localhost:3000/");
    println!();
    
    axum::serve(listener, app).await?;
    
    Ok(())
}