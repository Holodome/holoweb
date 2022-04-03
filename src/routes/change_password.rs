use crate::domain::credentials::Credentials;
use crate::domain::users::{UserName, UserPassword};
use crate::middleware::{reject_anonymous_users, Session};
use crate::services::{validate_credentials, AuthError};
use crate::startup::Pool;
use crate::utils::{e500, extract_errors, extract_infos, see_other};
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{route, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use actix_web_lab::middleware::from_fn;
use askama::Template;
use secrecy::{ExposeSecret, Secret};
use std::fmt::Formatter;

#[derive(Template)]
#[template(path = "change_password.html")]
struct PageTemplate {
    errors: Vec<String>,
    infos: Vec<String>,
    current_user_name: Option<UserName>,
}

#[route(
    "change_password",
    method = "GET",
    wrap = "from_fn(reject_anonymous_users)"
)]
pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let current_user_name = session.get_user_name().map_err(e500)?;
    let s = PageTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_name,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

#[derive(serde::Deserialize)]
pub struct FormData {
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
    InvalidNewPassword(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ChangePasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

#[route(
    "/change_password",
    method = "POST",
    wrap = "from_fn(reject_anonymous_users)"
)]
#[tracing::instrument(skip(form, pool))]
pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    user_name: web::ReqData<UserName>,
) -> Result<HttpResponse, InternalError<ChangePasswordError>> {
    let user_name = user_name.into_inner();
    if form.new_password.expose_secret() != form.repeat_new_password.expose_secret() {
        return Err(redirect(ChangePasswordError::RepeatPasswordDoesntMatch));
    }

    let credentials = Credentials {
        name: user_name.clone(),
        password: UserPassword::parse(form.current_password.clone())
            .map_err(|e| redirect(ChangePasswordError::InvalidCurrentPassword(e.into())))?,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        let e = match e {
            AuthError::InvalidCredentials(_) => {
                ChangePasswordError::InvalidCurrentPassword(e.into())
            }
            AuthError::UnexpectedError(_) => ChangePasswordError::UnexpectedError(e.into()),
        };
        return Err(redirect(e));
    }

    let new_password = UserPassword::parse(form.new_password.clone())
        .map_err(|e| redirect(ChangePasswordError::InvalidNewPassword(e.into())))?;
    crate::services::change_password(&pool, &user_name, &new_password)
        .await
        .map_err(|e| redirect(ChangePasswordError::UnexpectedError(e)))?;
    FlashMessage::info("Your password has been changed").send();
    Ok(see_other("/change_password"))
}

fn redirect(e: ChangePasswordError) -> InternalError<ChangePasswordError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/change_password"))
}
