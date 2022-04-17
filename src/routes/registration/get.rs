use crate::domain::users::UserID;
use crate::utils::render_template;

use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "registration.html")]
struct RegistrationTemplate {
    messages: IncomingFlashMessages,
}

#[tracing::instrument(skip(messages))]
pub async fn registration_form(
    messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    render_template(RegistrationTemplate { messages })
}
