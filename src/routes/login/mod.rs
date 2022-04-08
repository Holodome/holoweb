use actix_web::web;

mod get;
mod post;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::get().to(get::login_form))
            .route(web::post().to(post::login)),
    );
}
