use crate::domain::users::{
    Credentials, HashedUserPassword, PasswordError, UpdateUser, UserID, UserName, UserPassword,
};
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
use secrecy::{ExposeSecret, Secret};

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

#[derive(serde::Deserialize)]
pub struct ChangePasswordForm {
    current_password: Secret<String>,
    new_password: Secret<String>,
    repeat_new_password: Secret<String>,
}

#[derive(thiserror::Error)]
pub enum ChangePasswordError {
    #[error("Repeat password does not match new password")]
    RepeatPasswordDoesntMatch,
    #[error("Current password is incorrect")]
    InvalidCurrentPassword(#[source] anyhow::Error),
    #[error("New password is invalid")]
    InvalidNewPassword(#[source] PasswordError),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ChangePasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

#[tracing::instrument("Change password", skip(form, pool))]
pub async fn change_password(
    form: web::Form<ChangePasswordForm>,
    pool: web::Data<Pool>,
    user_id: UserID,
) -> Result<HttpResponse, InternalError<ChangePasswordError>> {
    if form.new_password.expose_secret() != form.repeat_new_password.expose_secret() {
        return Err(redirect_with_error(
            "/account/home",
            ChangePasswordError::RepeatPasswordDoesntMatch,
        ));
    }

    let user = get_user_by_id(&pool, &user_id)
        .map_err(|e| redirect_with_error("/account/home", ChangePasswordError::UnexpectedError(e)))?
        .ok_or_else(|| {
            redirect_with_error(
                "/account/home",
                ChangePasswordError::UnexpectedError(anyhow::anyhow!("Failed to get user")),
            )
        })?;

    let old_password = UserPassword::parse(form.current_password.clone()).map_err(|e| {
        redirect_with_error(
            "/account/home",
            ChangePasswordError::InvalidCurrentPassword(e.into()),
        )
    })?;

    let credentials = Credentials {
        name: user.name,
        password: old_password,
    };

    if let Err(e) = validate_credentials(credentials, &pool) {
        let e = match e {
            AuthError::InvalidCredentials(_) => {
                ChangePasswordError::InvalidCurrentPassword(e.into())
            }
            AuthError::UnexpectedError(_) => ChangePasswordError::UnexpectedError(e.into()),
        };
        return Err(redirect_with_error("/account/home", e));
    }

    let new_password = UserPassword::parse(form.new_password.clone()).map_err(|e| {
        redirect_with_error(
            "/account/home",
            ChangePasswordError::InvalidNewPassword(e.into()),
        )
    })?;
    let hashed_new_password = HashedUserPassword::parse(&new_password, &user.password_salt);

    let changeset = UpdateUser {
        id: &user_id,
        name: None,
        email: None,
        password: Some(&hashed_new_password),
        is_banned: None,
    };
    update_user(&pool, &changeset).map_err(|e| {
        redirect_with_error(
            "/account/home",
            ChangePasswordError::UnexpectedError(e.into()),
        )
    })?;

    FlashMessage::info("Your password has been changed").send();
    Ok(see_other("/account/home"))
}
