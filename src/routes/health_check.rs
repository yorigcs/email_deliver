use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use sqlx::PgPool;
use tracing::info;

async fn status(State(_pool): State<PgPool>) -> Json<Value> {
    info!("Okay");
    Json(json!({"status": "Ok"}))
}

pub fn health_check_routes() -> Router<PgPool> {
    Router::new().route("/health_check", get(status))
}
