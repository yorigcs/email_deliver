use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use crate::routes:: {status, subscribe};

pub async fn run(listener: TcpListener) -> std::io::Result<()>{
    axum::serve(listener, routes()).await.unwrap();
    Ok(())
}

pub fn routes() -> Router {
    Router::new()
        .route("/health_check", get(status))
        .route("/subscriptions", post(subscribe))

}
