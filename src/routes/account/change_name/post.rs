use crate::domain::credentials::Credentials;
use crate::domain::users::{UserID, UserName, UserPassword};
use crate::services::{get_user_by_id, get_user_by_name, validate_credentials, AuthError};
use crate::startup::Pool;
use crate::utils::{see_other};
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::Secret;
use std::fmt::Formatter;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_name: String,
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

#[tracing::instrument(skip(form, pool))]
pub async fn change_name(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    user_id: UserID,
) -> Result<HttpResponse, InternalError<ChangeNameError>> {
    let user_name = get_user_by_id(&pool, &user_id)
        .map_err(|e| redirect(ChangeNameError::UnexpectedError(e)))?
        .ok_or_else(|| {
            redirect(ChangeNameError::UnexpectedError(anyhow::anyhow!(
                "Failed to get user name"
            )))
        })?
        .name;

    let credentials = Credentials {
        name: user_name.clone(),
        password: UserPassword::parse(form.current_password.clone())
            .map_err(|e| redirect(ChangeNameError::InvalidCurrentPassword(e.into())))?,
    };
    if let Err(e) = validate_credentials(credentials, &pool) {
        let e = match e {
            AuthError::InvalidCredentials(_) => ChangeNameError::InvalidCurrentPassword(e.into()),
            AuthError::UnexpectedError(_) => ChangeNameError::UnexpectedError(e.into()),
        };
        return Err(redirect(e));
    }

    let new_user_name = UserName::parse(form.new_name.clone())
        .map_err(|e| redirect(ChangeNameError::InvalidName(e)))?;
    if get_user_by_name(&pool, &new_user_name)
        .map_err(|e| redirect(ChangeNameError::UnexpectedError(e)))?
        .is_some()
    {
        return Err(redirect(ChangeNameError::TakenName));
    }

    crate::services::change_name(&pool, &user_name, &new_user_name)
        .map_err(|e| redirect(ChangeNameError::UnexpectedError(e)))?;

    FlashMessage::info("Your name has been changed").send();
    Ok(see_other("/account"))
}

fn redirect(e: ChangeNameError) -> InternalError<ChangeNameError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/change_name"))
}
