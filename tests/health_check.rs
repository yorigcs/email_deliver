use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;
mod helpers;
use helpers::TestApp;

#[tokio::test]
async fn health_check_works() {
    let TestApp { db_pool, app } = TestApp::new().await;

    let response = app
        .with_state(db_pool.clone())
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({ "status": "Ok" }));
}
