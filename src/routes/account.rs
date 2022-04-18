use crate::domain::users::{Credentials, UpdateUser, UserID, UserName, UserPassword};
use crate::services::{
    get_user_by_id, get_user_by_name, update_user, validate_credentials, AuthError, UserError,
};
use crate::utils::{e500, redirect_with_error, render_template, see_other};
use std::fmt::Formatter;

use crate::domain::blog_posts::BlogPost;
use crate::domain::projects::Project;
use crate::Pool;
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use actix_web_lab::web::redirect;
use askama::Template;

struct ProjectInfo<'a> {
    id: &'a str,
    title: &'a str,
    brief: &'a str,
    role: &'a str,
}

struct BlogPostInfo<'a> {
    id: &'a str,
    title: &'a str,
    brief: &'a str,
    role: &'a str,
}

struct CommentInfo<'a> {
    project: &'a Project,
    blog_post: &'a BlogPost,
    date: &'a str,
    contents: &'a str,
}

#[derive(Template)]
#[template(path = "account.html")]
struct AccountPage<'a> {
    messages: IncomingFlashMessages,
    projects: Vec<ProjectInfo<'a>>,
    blog_posts: Vec<BlogPostInfo<'a>>,
    comments: Vec<CommentInfo<'a>>,
    name: &'a str,
    email: &'a str,
}

#[tracing::instrument(skip(pool, messages))]
pub async fn account(
    pool: web::Data<Pool>,
    user_id: UserID,
    messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let user = get_user_by_id(&pool, &user_id)
        .map_err(e500)?
        .ok_or_else(|| e500("Failed to get user name"))?;

    render_template(AccountPage {
        messages,
        projects: vec![],
        blog_posts: vec![],
        comments: vec![],
        name: user.name.as_ref(),
        email: user.email.as_ref(),
    })
}

#[derive(thiserror::Error)]
pub enum ChangeNameError {
    #[error("Current password is invalid")]
    InvalidCurrentPassword(#[source] anyhow::Error),
    #[error("Taken name")]
    TakenName,
    #[error("Invalid name")]
    InvalidName(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ChangeNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

#[derive(serde::Deserialize)]
pub struct ChangeNameForm {
    new_name: String,
}

#[tracing::instrument(skip(form, pool))]
pub async fn change_name(
    form: web::Form<ChangeNameForm>,
    pool: web::Data<Pool>,
    user_id: UserID,
) -> Result<HttpResponse, InternalError<ChangeNameError>> {
    let user_name = UserName::parse(form.0.new_name)
        .map_err(|e| redirect_with_error("/account/home", ChangeNameError::InvalidName(e)))?;

    let changeset = UpdateUser {
        id: &user_id,
        name: Some(&user_name),
        email: None,
        password: None,
        is_banned: None,
    };

    update_user(&pool, &changeset).map_err(|e| {
        redirect_with_error(
            "/account/home",
            match e {
                UserError::TakenName => ChangeNameError::TakenName,
                _ => ChangeNameError::UnexpectedError(e.into()),
            },
        )
    })?;
    FlashMessage::info("Your name has been changed").send();
    Ok(see_other("/account/home"))
}

#[tracing::instrument("Change email")]
pub async fn change_email() -> actix_web::Result<HttpResponse> {
    todo!()
}

#[tracing::instrument("Change password")]
pub async fn change_password() -> actix_web::Result<HttpResponse> {
    todo!()
}
