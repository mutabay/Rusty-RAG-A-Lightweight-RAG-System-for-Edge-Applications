mod routes;
mod utils;
mod services;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use routes::upload::upload;
use routes::query::query;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // load .env

    let app = Router::new()
        .route("/", get(root))  // 👈 THIS must exist
        .route("/upload", post(upload))
        .route("/query", post(query));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    
    println!("🚀 Server running at http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to DocuQuery API 👋"
    
}