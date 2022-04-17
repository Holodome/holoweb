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

/*
/health_check                      G
/                                  G
/registration                      GP
/login                             GP
/logout                            G
/blog_posts
   /all                            G
   /create                         GP
   /{id}/view                      G+L
   /{id}/edit                      GP+L
   /{id}/comments/create           P+L
   /{id}/comments/{id}/edit        P+L
/account                           +L
   /change_name                    GP
   /change_password                GP
   /home                           G
*/

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check::health_check))
        .service(
            web::resource("/logout")
                .wrap(from_fn(require_login))
                .route(web::get().to(logout::logout)),
        )
        .service(
            web::resource("/registration")
                .route(web::get().to(registration::get::registration_form))
                .route(web::post().to(registration::post::registration)),
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
                .service(
                    web::resource("/create")
                        .wrap(from_fn(require_login))
                        // .route(web::get().to(blog_posts::create::get::create_blog_post_form))
                        .route(web::post().to(blog_posts::create::post::create_blog_post)),
                )
                .route("/{post_id}/view", web::get().to(blog_posts::get::blog_post))
                .service(
                    web::resource("/{post_id}/edit")
                        .wrap(from_fn(require_login))
                        // .route(web::get().to(blog_posts::edit::get::edit_blog_post_form))
                        .route(web::post().to(blog_posts::edit::post::edit_blog_post)),
                )
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
        )
        .service(
            web::scope("/account")
                .wrap(from_fn(require_login))
                .route("/home", web::get().to(account::get::account)),
        );
}
