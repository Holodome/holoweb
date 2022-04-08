use crate::domain::users::UserID;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "create_blog_post.html")]
struct CreateBlogPostTemplate {
    current_user_id: Option<UserID>,
}

#[tracing::instrument("Create new blog post form")]
pub async fn create_blog_post_form(user_id: UserID) -> actix_web::Result<HttpResponse> {
    let s = CreateBlogPostTemplate {
        current_user_id: Some(user_id),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
