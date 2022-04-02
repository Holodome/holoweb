use crate::middleware::Session;
use actix_web::error::ErrorInternalServerError;
use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use askama::Template;
use crate::domain::users::UserName;

#[derive(Template)]
#[template(path = "account.html")]
struct AccountPage<'a> {
    current_user_name: &'a Option<UserName>,
    user_name: &'a str,
}

#[tracing::instrument(skip(session))]
pub async fn account(session: Session) -> actix_web::Result<HttpResponse> {
    let current_user_name = session
        .get_user_name()
        .map_err(ErrorInternalServerError)?;
    let user_name = current_user_name
        .as_ref()
        .expect("Failed to get user")
        .as_ref()
        .as_str();
    let s = AccountPage { current_user_name: &current_user_name, user_name }.render().unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
