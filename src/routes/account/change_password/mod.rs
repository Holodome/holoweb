use actix_web::web;

mod get;
mod post;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/change_password")
            .route(web::get().to(get::change_password_form))
            .route(web::post().to(post::change_password)),
    );
}
