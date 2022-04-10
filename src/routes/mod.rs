use crate::middleware::require_login;
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web_lab::middleware::from_fn;
use askama::Template;

mod account;
mod blog_posts;
mod health_check;
mod home;
mod login;
mod logout;
mod registration;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check::health_check))
        .service(
            web::resource("/logout")
                .wrap(from_fn(require_login))
                .route(web::get().to(logout::logout)),
        )
        .route("/", web::get().to(home::home))
        .service(
            web::resource("/registration")
                .route(web::get().to(registration::get::registration_form))
                .route(web::post().to(registration::post::registration)),
        )
        .service(
            web::resource("/login")
                .route(web::get().to(login::get::login_form))
                .route(web::post().to(login::post::login)),
        )
        .service(
            web::scope("/blog_posts")
                .route("/all", web::get().to(blog_posts::get::all_blog_posts))
                .route("/{post_id}/view", web::get().to(blog_posts::get::blog_post))
                .service(
                    web::resource("/create")
                        .wrap(from_fn(require_login))
                        .route(web::get().to(blog_posts::create::get::create_blog_post_form))
                        .route(web::post().to(blog_posts::create::post::create_blog_post)),
                )
                .service(
                    web::resource("/{post_id}/edit")
                        .wrap(from_fn(require_login))
                        .route(web::get().to(blog_posts::edit::get::edit_blog_post_form))
                        .route(web::post().to(blog_posts::edit::post::edit_blog_post)),
                ),
        )
        .service(
            web::scope("/account")
                .wrap(from_fn(require_login))
                .route("/home", web::get().to(account::get::account))
                .service(
                    web::resource("/change_name")
                        .route(web::get().to(account::change_name::get::change_name_form))
                        .route(web::post().to(account::change_name::post::change_name)),
                )
                .service(
                    web::resource("/change_password")
                        .route(web::get().to(account::change_password::get::change_password_form))
                        .route(web::post().to(account::change_password::post::change_password)),
                ),
        );
}

fn render_template<T: Template>(template: T) -> actix_web::Result<HttpResponse> {
    let s = template.render().map_err(e500)?;
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
