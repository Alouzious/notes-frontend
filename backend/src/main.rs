use axum::{
    Router,
    http::Method,
};
use std::net::SocketAddr;
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod routes;
mod models;

use db::get_db_pool;
use routes::note::notes_router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = get_db_pool().await;

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "http://localhost:5174".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .nest("/notes", notes_router(pool))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✅ Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
