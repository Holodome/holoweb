use crate::domain::users::UserID;
use crate::middleware::Session;
use crate::utils::{extract_errors, extract_infos};
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    errors: Vec<String>,
    infos: Vec<String>,
    current_user_id: Option<UserID>,
}

#[tracing::instrument(skip(flash_messages, session))]
pub(super) async fn login_form(
    flash_messages: IncomingFlashMessages,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let user_id = session.get_user_id().unwrap();
    let s = LoginTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_id: user_id,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
