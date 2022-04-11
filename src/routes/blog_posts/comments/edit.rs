use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::{CommentID, UpdateComment};
use crate::services::update_comment;
use crate::startup::Pool;
use crate::utils::{e500, see_other};
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct EditCommentQuery {
    contents: Option<String>,
    is_deleted: Option<bool>,
}

pub async fn edit_comment(
    pool: web::Data<Pool>,
    path: web::Path<(BlogPostID, CommentID)>,
    query: web::Query<EditCommentQuery>,
) -> actix_web::Result<HttpResponse> {
    let (post_id, comment_id) = path.into_inner();
    let changeset = UpdateComment {
        id: &comment_id,
        contents: query.0.contents.as_deref(),
        is_deleted: query.0.is_deleted,
    };
    update_comment(&pool, &changeset).map_err(e500)?;
    Ok(see_other(&format!("/blog_posts/{}", post_id)))
}
