use crate::utils::render_template;

use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    messages: IncomingFlashMessages,
}

#[tracing::instrument(skip(messages))]
pub async fn login_form(messages: IncomingFlashMessages) -> actix_web::Result<HttpResponse> {
    render_template(LoginTemplate { messages })
}
