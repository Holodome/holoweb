use crate::middleware::require_login;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

mod change_name;
mod change_password;
mod get;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .wrap(from_fn(require_login))
            .route("/home", web::get().to(get::account))
            .configure(change_password::configure)
            .configure(change_name::configure),
    );
}
