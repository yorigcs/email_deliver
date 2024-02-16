use axum::{extract::{Request}, http::{self, StatusCode}, body::Body};
use tower::ServiceExt;
use mime;
use sqlx::{PgConnection, Connection};
use email_deliver::configuration::get_configuration;
use email_deliver::startup::routes;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let body = String::from("name=le%20guin&email=ursula_le_guin%40gmail.com");
    let config = get_configuration().expect("Failed to read settings");
    let connection_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres");

    let response = routes()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/subscriptions")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED.as_ref())
                .body(Body::new(body))
                .unwrap()
        ).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions").
        fetch_one(&mut connection)
        .await
        .expect("Failed to fetch save subscription");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_valid_form_data() {
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error) in test_cases {
        let response = routes()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/subscriptions")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED.as_ref())
                    .body(Body::new(invalid_body.to_string()))
                    .unwrap()
            ).await.unwrap();
        assert_eq!(response.status(),
                   StatusCode::BAD_REQUEST,
                   "The API did not fail with status code 400 when the payload was {}.",
                   error
        );
    }
}