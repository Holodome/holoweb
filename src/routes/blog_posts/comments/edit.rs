use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::{CommentID, UpdateComment};
use crate::services::update_comment;
use crate::startup::Pool;
use crate::utils::{e500, see_other};
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct EditCommentForm {
    contents: String,
    is_deleted: bool,
}

pub async fn edit_comment(
    pool: web::Data<Pool>,
    path: web::Path<(BlogPostID, CommentID)>,
    form: web::Form<EditCommentForm>,
) -> actix_web::Result<HttpResponse> {
    let (post_id, comment_id) = path.into_inner();
    let changeset = UpdateComment {
        id: &comment_id,
        contents: Some(form.0.contents.as_str()),
        is_deleted: Some(form.0.is_deleted),
    };
    update_comment(&pool, &changeset).map_err(e500)?;
    Ok(see_other(&format!("/blog_posts/{}", post_id)))
}
