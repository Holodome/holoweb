use actix_web::web;

mod get;
mod post;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/registration")
            .route(web::get().to(get::registration_form))
            .route(web::post().to(post::registration)),
    );
}
