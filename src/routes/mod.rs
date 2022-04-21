use crate::middleware::{require_login, require_non_logged};
use crate::utils::see_other;
use actix_web::{web, HttpResponse};
use actix_web_lab::middleware::from_fn;

mod account;
mod blog_posts;
mod comments;
mod health_check;
mod login;
mod logout;
mod projects;
mod registration;

async fn redirect_to_blog_posts() -> HttpResponse {
    see_other("/blog_posts/all")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check::health_check))
        .route("/", web::get().to(redirect_to_blog_posts))
        .service(
            web::resource("/logout")
                .wrap(from_fn(require_login))
                .route(web::get().to(logout::logout)),
        )
        .service(
            web::resource("/registration")
                .wrap(from_fn(require_non_logged))
                .route(web::get().to(registration::registration_form))
                .route(web::post().to(registration::registration)),
        )
        .service(
            web::scope("/account")
                .wrap(from_fn(require_login))
                .route("/home", web::get().to(account::account))
                .route("/change_name", web::post().to(account::change_name))
                .route("/change_password", web::post().to(account::change_password))
                .route("/change_email", web::post().to(account::change_email)),
        )
        .service(
            web::resource("/login")
                .wrap(from_fn(require_non_logged))
                .route(web::get().to(login::login_form))
                .route(web::post().to(login::login)),
        )
        .service(
            web::scope("/blog_posts")
                .route("/all", web::get().to(blog_posts::all_blog_posts))
                .route("/{post_id}/view", web::get().to(blog_posts::blog_post))
                .service(
                    web::resource("/create")
                        .wrap(from_fn(require_login))
                        .route(web::get().to(blog_posts::create_blog_post_form))
                        .route(web::post().to(blog_posts::create_blog_post)),
                )
                .service(
                    web::resource("/{post_id}/edit")
                        .wrap(from_fn(require_login))
                        .route(web::get().to(blog_posts::edit_blog_post_form))
                        .route(web::post().to(blog_posts::edit_blog_post)),
                )
                .service(
                    web::resource("/{post_id}/comments/create")
                        .wrap(from_fn(require_login))
                        .route(web::post().to(comments::create_comment)),
                )
                .service(
                    web::resource("/{post_id}/comments/{comment_id}/edit")
                        .wrap(from_fn(require_login))
                        .route(web::post().to(comments::edit_comment)),
                ),
        );
}
