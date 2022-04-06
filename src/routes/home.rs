use crate::domain::users::{UserID};

use crate::utils::{extract_errors, extract_infos};

use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

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
    user_id: web::ReqData<UserID>,
) -> actix_web::Result<HttpResponse> {
    let s = HomeTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_id: Some(user_id.into_inner()),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
