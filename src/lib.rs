use axum::{Json, Router};
use axum::routing::get;
use serde_json::{Value,json};
pub async fn run() -> std::io::Result<()>{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, routes()).await.unwrap();
    Ok(())
}

pub fn routes() -> Router {
    Router::new()
        .route("/health_check", get(status))
}

async fn status() -> Json<Value> {
    Json(json!({"status": "Ok"}))
}