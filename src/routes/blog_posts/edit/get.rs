use crate::domain::blog_posts::{BlogPost, BlogPostID};
use crate::domain::users::UserID;
use crate::services::get_blog_post_by_id;
use crate::startup::Pool;
use crate::utils::{e500, render_template};
use actix_web::error::ErrorNotFound;

use actix_web::{web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "edit_blog_post.html")]
struct EditBlogPostTemplate {
    current_user_id: Option<UserID>,
    blog_post: BlogPost,
}

#[tracing::instrument("Create new blog post form", skip(pool))]
pub async fn edit_blog_post_form(
    pool: web::Data<Pool>,
    user_id: UserID,
    post_id: web::Path<BlogPostID>,
) -> actix_web::Result<HttpResponse> {
    render_template(EditBlogPostTemplate {
        current_user_id: Some(user_id),
        blog_post: get_blog_post_by_id(&pool, &post_id.into_inner())
            .map_err(e500)?
            .ok_or_else(|| ErrorNotFound("No blog post with such id"))?,
    })
}
