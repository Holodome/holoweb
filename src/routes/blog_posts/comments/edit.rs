use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::{CommentID, UpdateComment};
use crate::domain::users::UserID;
use crate::services::{get_comment_by_id, update_comment};
use crate::utils::see_other;
use crate::Pool;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};

#[derive(thiserror::Error)]
pub enum EditCommentError {
    #[error("Can't change others comment")]
    CantChangeOthersComment,
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for EditCommentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;
        error_chain_fmt(self, f)
    }
}

impl ResponseError for EditCommentError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::CantChangeOthersComment => StatusCode::FORBIDDEN,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct EditCommentForm {
    contents: String,
    is_deleted: bool,
}

pub async fn edit_comment(
    pool: web::Data<Pool>,
    path: web::Path<(BlogPostID, CommentID)>,
    form: web::Form<EditCommentForm>,
    current_user_id: UserID,
) -> Result<HttpResponse, EditCommentError> {
    let (post_id, comment_id) = path.into_inner();
    if let Some(comment) =
        get_comment_by_id(&pool, &comment_id).map_err(EditCommentError::UnexpectedError)?
    {
        if comment.author_id != current_user_id {
            return Err(EditCommentError::CantChangeOthersComment);
        }
    }
    let changeset = UpdateComment {
        id: &comment_id,
        contents: Some(form.0.contents.as_str()),
        is_deleted: Some(form.0.is_deleted),
    };
    update_comment(&pool, &changeset).map_err(EditCommentError::UnexpectedError)?;
    Ok(see_other(&format!("/blog_posts/{}", post_id)))
}
