use actix_web::web;

mod account;
mod blog_posts;
mod health_check;
mod home;
mod login;
mod logout;
mod registration;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(health_check::configure)
        .configure(blog_posts::configure)
        .configure(home::configure)
        .configure(login::configure)
        .configure(logout::configure)
        .configure(registration::configure)
        .configure(account::configure);
}
