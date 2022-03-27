#[macro_use]
extern crate diesel;

pub mod schema;
pub mod authentication;
pub mod config;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;
