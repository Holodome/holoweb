use crate::domain::users::{UserID, UserName};
use crate::middleware::{reject_anonymous_users};
use crate::services::get_user_by_id;
use crate::startup::Pool;


use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::ContentType;
use actix_web::{route, web, HttpResponse};
use actix_web_lab::middleware::from_fn;
use askama::Template;

#[derive(Template)]
#[template(path = "account.html")]
struct AccountPage<'a> {
    current_user_id: Option<&'a UserID>,
    user_name: &'a UserName,
}

#[tracing::instrument(skip(pool))]
#[route("/account", method = "GET", wrap = "from_fn(reject_anonymous_users)")]
pub async fn account(
    pool: web::Data<Pool>,
    user_id: web::ReqData<UserID>,
) -> actix_web::Result<HttpResponse> {
    let user_name = &get_user_by_id(&pool, &user_id)
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorInternalServerError(""))?
        .name;
    let s = AccountPage {
        current_user_id: Some(&user_id.into_inner()),
        user_name,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
