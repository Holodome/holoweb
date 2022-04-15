use crate::domain::blog_posts::{BlogPostID, UpdateBlogPost};
use crate::services::update_blog_post;
use crate::utils::{e500, see_other};
use crate::Pool;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct EditBlogPostFormData {
    title: String,
    brief: String,
    contents: String,
}
#[tracing::instrument("Edit blog post", skip(pool, form))]
pub async fn edit_blog_post(
    form: web::Form<EditBlogPostFormData>,
    pool: web::Data<Pool>,
    post_id: web::Path<BlogPostID>,
) -> actix_web::Result<HttpResponse> {
    let changeset = UpdateBlogPost {
        id: &post_id.into_inner(),
        title: Some(&form.title),
        brief: Some(&form.brief),
        contents: Some(&form.contents),
    };
    update_blog_post(&pool, &changeset).map_err(e500)?;
    Ok(see_other("/blog_posts/all"))
}
