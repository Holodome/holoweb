use actix_web::web;

mod change_name;
mod change_password;
mod get;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::get().to(get::account)))
        .configure(change_password::configure)
        .configure(change_name::configure);
}
