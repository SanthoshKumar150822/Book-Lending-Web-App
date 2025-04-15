mod database;
mod models;
mod routes;

use routes::routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Initialize database
    let pool = database::init_db().await?;
    println!("✅ Database connected successfully");

    // Set up routes
    let api = routes(pool);
    
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);
    
    warp::serve(api).run(addr).await;
    
    Ok(())
}