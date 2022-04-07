use actix_web::web;

mod get;
mod post;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/change_name")
            .route(web::get().to(get::change_name_form))
            .route(web::post().to(post::change_name)),
    );
}
