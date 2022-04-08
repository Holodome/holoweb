use crate::domain::blog_posts::NewBlogPost;
use crate::domain::users::UserID;
use crate::startup::Pool;
use crate::utils::{e500, see_other};
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct CreateBlogPostFormData {
    title: String,
    brief: String,
    contents: String,
}

#[tracing::instrument("Create new blog post", skip(pool, form))]
pub async fn create_blog_post(
    form: web::Form<CreateBlogPostFormData>,
    pool: web::Data<Pool>,
    user_id: web::ReqData<UserID>,
) -> actix_web::Result<HttpResponse> {
    let new_blog_post = NewBlogPost {
        title: form.0.title,
        brief: form.0.brief,
        contents: form.0.contents,
        author_id: user_id.into_inner(),
    };
    crate::services::insert_new_blog_post(&pool, &new_blog_post).map_err(e500)?;
    Ok(see_other("/blog_posts"))
}