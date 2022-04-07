use crate::domain::users::UserID;
use crate::middleware::Session;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "create_blog_post.html")]
struct CreateBlogPostTemplate {
    current_user_id: Option<UserID>,
}

#[tracing::instrument("Create new blog post form", skip(session))]
pub async fn create_blog_post_form(session: Session) -> actix_web::Result<HttpResponse> {
    let user_id = session.get_user_id().unwrap().unwrap();
    let s = CreateBlogPostTemplate {
        current_user_id: Some(user_id),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
