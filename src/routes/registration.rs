use crate::domain::users::{NewUser, NewUserError, PasswordError, UserName};
use crate::middleware::Session;
use crate::services::{get_user_by_name, insert_new_user};
use crate::startup::Pool;
use crate::utils::{e500, extract_errors, see_other};
use actix_web::error::{ErrorInternalServerError, InternalError};
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama::Template;
use secrecy::{ExposeSecret, Secret};
use std::fmt::Formatter;

#[derive(Template)]
#[template(path = "registration.html")]
struct RegistrationTemplate {
    errors: Vec<String>,
    current_user_name: Option<UserName>,
}

#[tracing::instrument(skip(flash_messages, session))]
pub async fn registration_form(
    flash_messages: IncomingFlashMessages,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let current_user_name = session.get_user_name().map_err(e500)?;

    let s = RegistrationTemplate {
        errors: extract_errors(&flash_messages),
        current_user_name,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

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

    let conn = pool
        .get()
        .map_err(|e| registration_redirect(RegistrationError::UnexpectedError(e.into())))?;
    if get_user_by_name(&conn, &new_user.name)
        .map_err(|e| registration_redirect(e.into()))?
        .is_some()
    {
        return Err(registration_redirect(RegistrationError::TakenName));
    }

    match insert_new_user(&conn, &new_user) {
        Ok(user) => {
            session.renew();
            session
                .insert_user_name(user.name)
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
