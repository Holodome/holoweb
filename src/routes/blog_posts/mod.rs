use actix_web::web;
mod get;

mod create;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog_posts")
            .service(web::resource("/").route(web::get().to(get::blog_posts)))
            .configure(create::configure),
    );
}
