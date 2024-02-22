use axum::Router;
use email_deliver::configuration::{DatabaseSettings, get_configuration};
use email_deliver::telemetry::init_tracing;
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    init_tracing();
});
pub struct TestApp {
    pub db_pool: PgPool,
    pub app: Router<PgPool>,
}

impl TestApp {
    pub async fn new() -> TestApp {
        let mut cfg = get_configuration().expect("Failed to load configuration.");
        Lazy::force(&TRACING);
        cfg.database.database_name = Uuid::new_v4().to_string();
        let db_pool = configure_database(&cfg.database).await;

        let app = email_deliver::startup::app();

        TestApp { db_pool, app }
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    pool
}