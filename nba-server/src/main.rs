mod load_data;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    load_data::load().await;

    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
