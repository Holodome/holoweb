#[macro_use]
extern crate diesel;

mod domain;
mod schema;

mod config;
mod error_handlers;
mod routes;
mod services;
mod startup;
mod telemetry;

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
