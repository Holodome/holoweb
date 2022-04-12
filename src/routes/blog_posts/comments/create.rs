use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::NewComment;
use crate::domain::users::UserID;
use crate::services::insert_new_comment;
use crate::startup::Pool;
use crate::utils::{e500, see_other};
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct CreateCommentFormData {
    contents: String,
}

pub async fn create_comment(
    pool: web::Data<Pool>,
    user_id: UserID,
    post_id: web::Path<BlogPostID>,
    query: web::Form<CreateCommentFormData>,
) -> actix_web::Result<HttpResponse> {
    let post_id = post_id.into_inner();
    let new_comment = NewComment {
        author_id: &user_id,
        post_id: &post_id,
        parent_id: None,
        contents: &query.0.contents,
    };
    insert_new_comment(&pool, &new_comment).map_err(e500)?;
    Ok(see_other(&format!("/blog_posts/{}", post_id)))
}
