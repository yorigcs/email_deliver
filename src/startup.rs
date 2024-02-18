use crate::routes::{health_check_routes, subscriptions_routes};
use crate::telemetry::init_tracing;
use axum::body::Body;
use axum::http::Request;
use axum::Router;
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;

pub async fn run(listener: TcpListener, pool: PgPool) -> std::io::Result<()> {
    init_tracing();
    tracing::debug!("Server listening on {}", listener.local_addr().unwrap());

    let app = app().with_state(pool.clone());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub fn app() -> Router<PgPool> {
    Router::new()
        .merge(subscriptions_routes())
        .merge(health_check_routes())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = uuid::Uuid::new_v4();
                tracing::span!(
                    Level::DEBUG,
                    "request",
                    method = display(request.method()),
                    uri = display(request.uri()),
                    version = debug(request.version()),
                    request_id = display(request_id)
                )
            }),
        )
}
