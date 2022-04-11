use crate::domain::users::UserID;

use crate::utils::render_template;

use crate::middleware::FlashMessages;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    flash_messages: FlashMessages,
    current_user_id: Option<UserID>,
}

#[tracing::instrument(skip(flash_messages))]
pub async fn home(
    flash_messages: IncomingFlashMessages,
    user_id: Option<UserID>,
) -> actix_web::Result<HttpResponse> {
    render_template(HomeTemplate {
        flash_messages: flash_messages.into(),
        current_user_id: user_id,
    })
}
