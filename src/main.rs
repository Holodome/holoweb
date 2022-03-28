use holosite::{telemetry, startup, config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("holosite".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    tracing::info!("Initialized logging");

    let config = config::get_config().expect("Failed to get config");
    tracing::info!("Loaded config: {:?}", config);

    let app = startup::Application::build(config).await?;
    app.run_until_stopped().await?;
    Ok(())
}
