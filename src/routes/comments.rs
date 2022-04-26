use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::NewComment;
use crate::domain::comments::{CommentID, UpdateComment};
use crate::domain::users::UserID;
use crate::middleware::Session;
use crate::services::insert_new_comment;
use crate::services::{get_comment_by_id, update_comment};
use crate::utils::{e500, redirect_with_error, see_other};
use crate::Pool;
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct CreateCommentFormData {
    contents: String,
    reply_to_id: Option<CommentID>,
}

#[tracing::instrument("Create comment", skip(pool, form))]
pub async fn create_comment(
    pool: web::Data<Pool>,
    user_id: UserID,
    post_id: web::Path<BlogPostID>,
    form: web::Form<CreateCommentFormData>,
) -> actix_web::Result<HttpResponse> {
    let post_id = post_id.into_inner();
    let new_comment = NewComment {
        author_id: &user_id,
        post_id: &post_id,
        parent_id: form.0.reply_to_id.as_ref(),
        contents: &form.0.contents,
    };
    let new_comment = insert_new_comment(&pool, &new_comment).map_err(e500)?;
    Ok(see_other(&format!(
        "/blog_posts/{}/view#comment-{}",
        post_id, new_comment.id
    )))
}

#[derive(thiserror::Error)]
pub enum EditCommentError {
    #[error("Invalid CSRF token")]
    CSRFError,
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

#[derive(serde::Deserialize)]
pub struct EditCommentForm {
    contents: String,
    csrf_token: Secret<String>,
}

#[tracing::instrument("Edit comment", skip(pool, form, session))]
pub async fn edit_comment(
    pool: web::Data<Pool>,
    path: web::Path<(BlogPostID, CommentID)>,
    form: web::Form<EditCommentForm>,
    current_user_id: UserID,
    session: Session,
) -> Result<HttpResponse, InternalError<EditCommentError>> {
    let (post_id, comment_id) = path.into_inner();
    let redirect = |e| {
        redirect_with_error(
            &format!("/blog_posts/{}/view#comment-{}", post_id, comment_id),
            e,
        )
    };

    if form.csrf_token.expose_secret()
        != session
            .get_csrf_token()
            .map_err(EditCommentError::UnexpectedError)
            .map_err(redirect)?
            .expose_secret()
    {
        return Err(redirect(EditCommentError::CSRFError));
    }

    if let Some(comment) = get_comment_by_id(&pool, &comment_id)
        .map_err(EditCommentError::UnexpectedError)
        .map_err(redirect)?
    {
        if comment.author_id != current_user_id {
            return Err(redirect(EditCommentError::CantChangeOthersComment));
        }
    }
    let changeset = UpdateComment {
        id: &comment_id,
        contents: Some(form.0.contents.as_str()),
        is_deleted: None,
    };
    update_comment(&pool, &changeset)
        .map_err(EditCommentError::UnexpectedError)
        .map_err(redirect)?;

    Ok(see_other(&format!(
        "/blog_posts/{}/view#comment-{}",
        post_id, comment_id
    )))
}

#[tracing::instrument("Delete comment", skip(pool))]
pub async fn delete_comment(
    pool: web::Data<Pool>,
    path: web::Path<(BlogPostID, CommentID)>,
    current_user_id: UserID,
) -> Result<HttpResponse, InternalError<EditCommentError>> {
    let (post_id, comment_id) = path.into_inner();
    let redirect = |e| {
        redirect_with_error(
            &format!("/blog_posts/{}/view#comment-{}", post_id, comment_id),
            e,
        )
    };

    // TODO: CSRF

    if let Some(comment) = get_comment_by_id(&pool, &comment_id)
        .map_err(EditCommentError::UnexpectedError)
        .map_err(redirect)?
    {
        if comment.author_id != current_user_id {
            return Err(redirect(EditCommentError::CantChangeOthersComment));
        }
    }
    let changeset = UpdateComment {
        id: &comment_id,
        contents: None,
        is_deleted: Some(true),
    };
    update_comment(&pool, &changeset)
        .map_err(EditCommentError::UnexpectedError)
        .map_err(redirect)?;
    Ok(see_other(&format!(
        "/blog_posts/{}/view#comment-{}",
        post_id, comment_id
    )))
}
