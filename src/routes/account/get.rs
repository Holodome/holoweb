use crate::domain::users::{UserID, UserName};
use crate::services::get_user_by_id;
use crate::utils::{e500, render_template};

use crate::Pool;
use actix_web::{web, HttpResponse};
use askama::Template;

#[tracing::instrument(skip(pool))]
pub async fn account(pool: web::Data<Pool>, user_id: UserID) -> actix_web::Result<HttpResponse> {
    let user_name = get_user_by_id(&pool, &user_id)
        .map_err(e500)?
        .ok_or_else(|| e500("Failed to get user name"))?
        .name;

    render_template(AccountPage {
        current_user_id: Some(user_id),
        user_name,
    })
}

#[derive(Template)]
#[template(path = "account.html")]
struct AccountPage {
    current_user_id: Option<UserID>,
    user_name: UserName,
}
