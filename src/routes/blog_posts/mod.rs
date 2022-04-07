use actix_web::web;
mod get;

mod create;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(create::configure)
        .service(web::resource("/blog_posts").route(web::get().to(get::blog_posts)));
}
