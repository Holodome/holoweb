use actix_web::web;
mod create;
mod edit;
mod get;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog_posts")
            .service(web::resource("/{post_id}").route(web::get().to(get::blog_post)))
            .service(web::resource("/all").route(web::get().to(get::all_blog_posts)))
            .configure(create::configure),
    );
}
