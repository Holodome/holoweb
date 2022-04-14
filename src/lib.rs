#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate core;

use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};

pub mod config;
pub mod domain;
pub mod error_handlers;
pub mod markdown;
pub mod middleware;
pub mod routes;
pub mod schema;
pub mod services;
pub mod startup;
pub mod telemetry;
pub mod utils;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
