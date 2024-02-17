use axum::{ Router, extract::{Form, State}};
use axum::http::StatusCode;
use axum::routing::post;
use serde::Deserialize;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(State(pool): State<PgPool>, Form(form): Form<FormData>) -> StatusCode {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email,name,subscribed_at)
        VALUES ($1,$2,$3,$4)
         "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    ).execute(&pool).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

}

pub fn subscriptions_routes() -> Router<PgPool> {
    Router::new()
        .route("/subscriptions", post(subscribe))
}