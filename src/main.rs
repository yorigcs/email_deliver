use email_deliver::configuration::get_configuration;
use email_deliver::startup::run;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to load configuration.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = dbg!(format!("127.0.0.1:{}", configuration.application_port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    run(listener, pool).await
}
