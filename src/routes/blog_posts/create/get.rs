use crate::domain::users::UserID;

use crate::utils::render_template;
use actix_web::HttpResponse;
use askama::Template;

// #[derive(Template)]
// #[template(path = "create_blog_post.html")]
// struct CreateBlogPostTemplate {
//     current_user_id: Option<UserID>,
// }
//
// #[tracing::instrument("Create new blog post form")]
// pub async fn create_blog_post_form(user_id: UserID) -> actix_web::Result<HttpResponse> {
//     render_template(CreateBlogPostTemplate {
//         current_user_id: Some(user_id),
//     })
// }
