use axum::{
    Router,
    http::Method,
};
use std::net::SocketAddr;
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use http::HeaderValue;

mod db;
mod routes;
mod models;

use db::get_db_pool;
use routes::note::notes_router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    println!("🔌 Connecting to database...");
    let pool = get_db_pool().await;
    println!("✅ Database connected successfully!");

    // Configure CORS to allow your frontend domains
    let cors = CorsLayer::new()
        .allow_origin([
            "https://notes-admin-panel.vercel.app".parse::<HeaderValue>().unwrap(),
            "https://notes-frontend-cyan-six.vercel.app".parse::<HeaderValue>().unwrap(),
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://localhost:5174".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any)
        .allow_credentials(false);

    let app = Router::new()
        .nest("/notes", notes_router(pool))
        .layer(cors);

    // Bind to 0.0.0.0 to accept external connections (required for Render)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("✅ Server running at http://{}", addr);
    println!("🌍 Ready to accept connections from external sources");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}