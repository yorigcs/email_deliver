use email_deliver::startup::run;
use email_deliver::configuration::get_configuration;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to load configuration.");
    let address = dbg!(format!("127.0.0.1:{}", configuration.application_port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    run(listener).await
}