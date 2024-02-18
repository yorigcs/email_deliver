use axum::http::StatusCode;
use axum::routing::post;
use axum::{
    extract::{Form, State},
    Router,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(State(pool): State<PgPool>, Form(form): Form<FormData>) -> StatusCode {
    info!("Adding {} {} as a new subscriber.", form.email, form.name);
    match insert_subscriber(pool, &form).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn insert_subscriber(pool: PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    info!("Saving new subscriber details in the database");
    sqlx::query!(
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
    .map_err(|e| {
        error!(" Failed to execute query {}", e);
        e
    })?;
    Ok(())
}
pub fn subscriptions_routes() -> Router<PgPool> {
    Router::new().route("/subscriptions", post(subscribe))
}
