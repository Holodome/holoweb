use crate::domain::users::UserID;
use crate::utils::render_template;

use crate::middleware::FlashMessages;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "change_password.html")]
struct PageTemplate {
    flash_messages: FlashMessages,
    current_user_id: Option<UserID>,
}

pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
    user_id: UserID,
) -> actix_web::Result<HttpResponse> {
    render_template(PageTemplate {
        flash_messages: flash_messages.into(),
        current_user_id: Some(user_id),
    })
}
