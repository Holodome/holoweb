use crate::middleware::require_login;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

pub mod change_name;
pub mod change_password;
pub mod get;
