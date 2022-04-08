use crate::domain::users::UserID;

use crate::utils::{extract_errors, extract_infos};

use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(home)));
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    errors: Vec<String>,
    infos: Vec<String>,
    current_user_id: Option<UserID>,
}

#[tracing::instrument(skip(flash_messages))]
pub async fn home(
    flash_messages: IncomingFlashMessages,
    user_id: Option<UserID>,
) -> actix_web::Result<HttpResponse> {
    let s = HomeTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_id: user_id,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
