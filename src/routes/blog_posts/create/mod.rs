use crate::middleware::require_login;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

pub mod get;
pub mod post;
