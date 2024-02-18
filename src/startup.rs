use crate::routes::{health_check_routes, subscriptions_routes};
use axum::Router;
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub async fn run(listener: TcpListener, pool: PgPool) -> std::io::Result<()> {
    let app = app().with_state(pool.clone());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "email_deliver=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::debug!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub fn app() -> Router<PgPool> {
    Router::new()
        .merge(subscriptions_routes())
        .merge(health_check_routes())
        .layer(TraceLayer::new_for_http())
}
