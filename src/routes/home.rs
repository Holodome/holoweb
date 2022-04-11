use crate::domain::users::UserID;

use crate::utils::{extract_errors, extract_infos, render_template};

use actix_web::HttpResponse;
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
    user_id: Option<UserID>,
) -> actix_web::Result<HttpResponse> {
    render_template(HomeTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_id: user_id,
    })
}
