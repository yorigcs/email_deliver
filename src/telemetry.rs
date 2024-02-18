use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub fn init_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "email_deliver=debug,tower_http=debug".into());
    let formatting_layer = tracing_subscriber::fmt::layer().json().flatten_event(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .init();
}
