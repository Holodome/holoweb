use crate::domain::UserName;
use crate::middleware::Session;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    current_user_name: Option<UserName>,
}

#[tracing::instrument(skip(session))]
pub async fn home(session: Session) -> actix_web::Result<HttpResponse> {
    let current_user_name = session.get_user_name().map_err(ErrorInternalServerError)?;
    let s = HomeTemplate { current_user_name }.render().unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
