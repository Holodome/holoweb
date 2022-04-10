use crate::domain::users::UserID;
use crate::routes::render_template;
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

#[tracing::instrument(skip(flash_messages))]
pub async fn login_form(
    flash_messages: IncomingFlashMessages,
    user_id: Option<UserID>,
) -> actix_web::Result<HttpResponse> {
    render_template(LoginTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_id: user_id,
    })
}
