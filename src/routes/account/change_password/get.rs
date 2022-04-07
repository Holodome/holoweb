use crate::domain::users::UserID;
use crate::middleware::Session;
use crate::utils::{extract_errors, extract_infos};
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "change_password.html")]
struct PageTemplate {
    errors: Vec<String>,
    infos: Vec<String>,
    current_user_id: Option<UserID>,
}

pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let user_id = session.get_user_id().unwrap().unwrap();
    let s = PageTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_id: Some(user_id),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
