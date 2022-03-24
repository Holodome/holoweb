#[macro_use]
extern crate diesel;

use std::net::TcpListener;
use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::{middleware, web, App, HttpServer};
use askama::filters::format;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use secrecy::Secret;
use crate::startup::{run, Pool};

mod domain;
mod router;
mod schema;

mod config;
mod error_handlers;
mod routes;
#[allow(dead_code)]
mod services;
mod templates;
mod startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or("LOG_LEVEL", "website=trace,actix_web=debug"),
    );
    dotenv::dotenv().ok();

    log::info!("Initialized logging");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL expected");

    let pool: Pool = Pool::builder()
        .build(ConnectionManager::new(database_url))
        .expect("Failed to create db pool");

    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    run(listener, pool)?.await?;
    Ok(())
}
