use axum::http::StatusCode;
use axum::routing::post;
use axum::{
    extract::{Form, State},
    Router,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(State(pool): State<PgPool>, Form(form): Form<FormData>) -> StatusCode {
    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - Adding {} {} as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    tracing::info!(
        "request_id {} - Saving new subscription details in db...",
        request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email,name,subscribed_at)
        VALUES ($1,$2,$3,$4)
         "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&pool)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscription details have been saved.",
                request_id
            );
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query {}", request_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub fn subscriptions_routes() -> Router<PgPool> {
    Router::new().route("/subscriptions", post(subscribe))
}
