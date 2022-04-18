use crate::error_handlers::redirect_on_same_page;
use crate::middleware::require_login;
use actix_web::middleware::ErrorHandlers;
use actix_web::{http, web};
use actix_web_lab::middleware::from_fn;

mod account;
mod blog_posts;
mod health_check;
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
        .service(
            web::resource("/registration")
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
                .wrap(
                    ErrorHandlers::new()
                        .handler(
                            http::StatusCode::INTERNAL_SERVER_ERROR,
                            redirect_on_same_page,
                        )
                        .handler(http::StatusCode::BAD_REQUEST, redirect_on_same_page),
                )
                .route(web::get().to(login::get::login_form))
                .route(web::post().to(login::post::login)),
        )
        .service(
            web::scope("/blog_posts")
                .route("/all", web::get().to(blog_posts::get::all_blog_posts))
                .route("/{post_id}/view", web::get().to(blog_posts::get::blog_post))
                .service(
                    web::resource("/{post_id}/comments/create")
                        .wrap(from_fn(require_login))
                        .route(web::post().to(blog_posts::comments::create::create_comment)),
                )
                .service(
                    web::resource("/{post_id}/comments/{comment_id}/edit")
                        .wrap(from_fn(require_login))
                        .route(web::post().to(blog_posts::comments::edit::edit_comment)),
                ),
        );
}
