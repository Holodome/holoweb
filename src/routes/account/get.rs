use crate::domain::users::{UserID, UserName};
use crate::services::get_user_by_id;
use crate::utils::{e500, render_template};

use crate::Pool;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "account.html")]
struct AccountPage {
    messages: IncomingFlashMessages,
    user_name: UserName,
}

#[tracing::instrument(skip(pool, messages))]
pub async fn account(
    pool: web::Data<Pool>,
    user_id: UserID,
    messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let user_name = get_user_by_id(&pool, &user_id)
        .map_err(e500)?
        .ok_or_else(|| e500("Failed to get user name"))?
        .name;

    render_template(AccountPage {
        messages,
        user_name,
    })
}
