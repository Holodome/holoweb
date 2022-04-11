use crate::domain::users::{Credentials, UserID, UserPassword};
use crate::services::{get_user_by_id, validate_credentials, AuthError};
use crate::startup::Pool;
use crate::utils::see_other;
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use std::fmt::Formatter;

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

#[tracing::instrument(skip(form, pool))]
pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    user_id: UserID,
) -> Result<HttpResponse, InternalError<ChangePasswordError>> {
    let user_name = get_user_by_id(&pool, &user_id)
        .map_err(|e| redirect(ChangePasswordError::UnexpectedError(e)))?
        .ok_or_else(|| {
            redirect(ChangePasswordError::UnexpectedError(anyhow::anyhow!(
                "Failed to get user name"
            )))
        })?
        .name;

    if form.new_password.expose_secret() != form.repeat_new_password.expose_secret() {
        return Err(redirect(ChangePasswordError::RepeatPasswordDoesntMatch));
    }

    let credentials = Credentials {
        name: user_name.clone(),
        password: UserPassword::parse(form.current_password.clone())
            .map_err(|e| redirect(ChangePasswordError::InvalidCurrentPassword(e.into())))?,
    };
    if let Err(e) = validate_credentials(credentials, &pool) {
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
        .map_err(|e| redirect(ChangePasswordError::UnexpectedError(e)))?;
    FlashMessage::info("Your password has been changed").send();
    Ok(see_other("/account"))
}

fn redirect(e: ChangePasswordError) -> InternalError<ChangePasswordError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/change_password"))
}
