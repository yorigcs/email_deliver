use axum::Router;
use email_deliver::configuration::get_configuration;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
pub struct TestApp {
    pub db_pool: PgPool,
    pub app: Router<PgPool>,
}

impl TestApp {
    pub async fn new() -> TestApp {
        let configuration = get_configuration().expect("Failed to load configuration.");

        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");

        let app = email_deliver::startup::app();

        TestApp { db_pool, app }
    }
}
