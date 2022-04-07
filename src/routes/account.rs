use crate::domain::users::{UserID, UserName};
use crate::middleware::{require_login, Session};
use crate::services::get_user_by_id;
use crate::startup::Pool;

use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::ContentType;
use actix_web::{route, web, HttpResponse};
use actix_web_lab::middleware::from_fn;
use askama::Template;

#[tracing::instrument(skip(pool, session))]
#[route("/account", method = "GET", wrap = "from_fn(require_login)")]
pub async fn account(pool: web::Data<Pool>, session: Session) -> actix_web::Result<HttpResponse> {
    let user_id = session.get_user_id().unwrap().unwrap();
    let user_name = get_user_by_id(&pool, &user_id)
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorInternalServerError("W"))?
        .name;
    let s = AccountPage {
        current_user_id: Some(user_id),
        user_name,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

#[derive(Template)]
#[template(path = "account.html")]
struct AccountPage {
    current_user_id: Option<UserID>,
    user_name: UserName,
}
