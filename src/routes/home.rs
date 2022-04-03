use crate::domain::users::UserName;
use crate::middleware::Session;
use crate::utils::{extract_errors, extract_infos};
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    errors: Vec<String>,
    infos: Vec<String>,
    current_user_name: Option<UserName>,
}

#[tracing::instrument(skip(session, flash_messages))]
pub async fn home(
    session: Session,
    flash_messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let current_user_name = session.get_user_name().map_err(ErrorInternalServerError)?;
    let s = HomeTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_name,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
