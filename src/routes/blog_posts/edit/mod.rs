use actix_web::web;

mod get;
mod post;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/edit")
            // .route(web::get().to(get::))
            // .route(web::post().to(post::login)),
    );
}
