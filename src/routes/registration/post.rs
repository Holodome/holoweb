use crate::domain::users::{NewUser, NewUserError, PasswordError};
use crate::middleware::Session;
use crate::services::{get_user_by_name, insert_new_user};
use crate::startup::Pool;
use crate::utils::see_other;
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use std::fmt::Formatter;

#[derive(thiserror::Error)]
pub enum RegistrationError {
    #[error("Invalid name")]
    InvalidName(#[source] anyhow::Error),
    #[error("Name is already taken")]
    TakenName,
    #[error("Invalid password")]
    InvalidPassword(#[source] PasswordError),
    #[error("Passwords don't match")]
    PasswordsDontMatch,
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for RegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

#[derive(serde::Deserialize)]
pub struct RegistrationFormData {
    name: String,
    password: Secret<String>,
    repeat_password: Secret<String>,
}

impl TryFrom<RegistrationFormData> for NewUser {
    type Error = RegistrationError;

    fn try_from(value: RegistrationFormData) -> Result<Self, Self::Error> {
        let dont_match = value.password.expose_secret() != value.repeat_password.expose_secret();
        let user = NewUser::parse(value.name, value.password).map_err(|e| match e {
            NewUserError::NameError(e) => RegistrationError::InvalidName(e),
            NewUserError::PasswordError(e) => RegistrationError::InvalidPassword(e),
        })?;

        if dont_match {
            return Err(RegistrationError::PasswordsDontMatch);
        }

        Ok(user)
    }
}

#[tracing::instrument("Registration", skip(form, pool, session))]
pub async fn registration(
    form: web::Form<RegistrationFormData>,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse, InternalError<RegistrationError>> {
    let new_user: NewUser =
        RegistrationFormData::try_into(form.0).map_err(registration_redirect)?;

    if get_user_by_name(&pool, &new_user.name)
        .map_err(|e| registration_redirect(e.into()))?
        .is_some()
    {
        return Err(registration_redirect(RegistrationError::TakenName));
    }

    match insert_new_user(&pool, &new_user) {
        Ok(user) => {
            session.renew();
            session
                .insert_user_id(user.id)
                .map_err(|e| registration_redirect(RegistrationError::UnexpectedError(e.into())))?;
            Ok(see_other("/"))
        }
        Err(e) => Err(registration_redirect(RegistrationError::UnexpectedError(e))),
    }
}

fn registration_redirect(e: RegistrationError) -> InternalError<RegistrationError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/registration"))
}
