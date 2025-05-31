mod routes;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use routes::upload::upload;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/upload", post(upload));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("🚀 Server running at http://localhost:8000");
    axum::serve(listener, app).await.unwrap();

}

async fn root() -> &'static str {
    "Welcome to DocuQuery API 👋"
}
