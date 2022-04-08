use crate::domain::users::UserID;
use crate::utils::extract_errors;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "registration.html")]
struct RegistrationTemplate {
    errors: Vec<String>,
    current_user_id: Option<UserID>,
}

#[tracing::instrument(skip(flash_messages))]
pub async fn registration_form(
    flash_messages: IncomingFlashMessages,
    user_id: Option<UserID>,
) -> Result<HttpResponse, actix_web::Error> {
    let s = RegistrationTemplate {
        errors: extract_errors(&flash_messages),
        current_user_id: user_id,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
