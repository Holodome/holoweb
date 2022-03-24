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
#[allow(dead_code)]
mod services;
mod startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or("LOG_LEVEL", "holosite=trace,actix_web=debug"),
    );
    log::info!("Initialized logging");

    let config = config::get_config().expect("Failed to get config");
    log::info!("Loaded config: {:?}", config);

    let pool: Pool = Pool::builder()
        .build(ConnectionManager::new(config.database_path))
        .expect("Failed to create db pool");

    let address = format!("127.0.0.1:{}", config.application_port);
    log::info!("Starting server on {:?}", &address);
    let listener = TcpListener::bind(address)?;

    run(listener, pool)?.await?;
    Ok(())
}
