use crate::middleware::require_login;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

mod get;
mod post;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .wrap(from_fn(require_login))
            .route(web::get().to(get::create_blog_post_form))
            .route(web::post().to(post::create_blog_post)),
    );
}
