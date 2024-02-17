use axum::Router;
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;
use crate::routes::{health_check_routes, subscriptions_routes};

pub async fn run(listener: TcpListener, pool: PgPool) -> std::io::Result<()>{
    let app = app().with_state(pool.clone());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub fn app() -> Router<PgPool> {
    Router::new()
        .merge(subscriptions_routes())
        .merge(health_check_routes())
}
