use axum::{extract::{Request}, http::{self, StatusCode}, body::Body};
use tower::ServiceExt;
use mime;
use sqlx::{Pool, Postgres};

mod helpers;
use helpers::TestApp;

#[sqlx::test]
async fn subscribe_returns_a_200_for_valid_form_data(pool: Pool<Postgres>) {
    let body = String::from("name=yori%20galisteu&email=yorigcs%40gmail.com");
    let TestApp{app, .. }  = TestApp::new().await;

    let response = app.with_state(pool.clone())
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/subscriptions")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::new(body))
                .unwrap()
        ).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch save subscription");

    assert_eq!(saved.email, "yorigcs@gmail.com");
    assert_eq!(saved.name, "yori galisteu");

}

