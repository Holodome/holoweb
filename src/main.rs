#[macro_use]
extern crate diesel;

use crate::startup::{run, Pool};
use diesel::r2d2::ConnectionManager;
use std::net::TcpListener;

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
    let subscriber = telemetry::get_subscriber("holosite".into(), "info".into());
    telemetry::init_subscriber(subscriber);
    tracing::info!("Initialized logging");

    let config = config::get_config().expect("Failed to get config");
    tracing::info!("Loaded config: {:?}", config);

    let pool: Pool = Pool::builder()
        .build(ConnectionManager::new(config.database_path))
        .expect("Failed to create db pool");

    let address = format!("127.0.0.1:{}", config.application_port);
    tracing::info!("Starting server on {:?}", &address);
    let listener = TcpListener::bind(address)?;

    run(listener, pool)?.await?;
    Ok(())
}
