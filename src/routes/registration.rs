use crate::domain::{NewUser, NewUserError, PasswordError};
use crate::routes::LoginError;
use crate::session::Session;
use crate::startup::Pool;
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use secrecy::{ExposeSecret, Secret};
use std::fmt::Formatter;

#[derive(Template)]
#[template(path = "registration.html")]
struct RegistrationTemplate {
    errors: Vec<String>,
}

#[tracing::instrument(skip(flash_messages))]
pub async fn registration_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let errors: Vec<String> = flash_messages
        .iter()
        .map(|m| m.content().to_string())
        .collect();
    let s = RegistrationTemplate { errors }.render().unwrap();
    HttpResponse::Ok().content_type(ContentType::html()).body(s)
}

#[derive(thiserror::Error)]
pub enum RegistrationError {
    #[error("Invalid name")]
    InvalidName(#[source] anyhow::Error),
    #[error("Name is already taken")]
    TakenName(#[source] anyhow::Error),
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
        if value.password.expose_secret() != value.repeat_password.expose_secret() {
            return Err(RegistrationError::PasswordsDontMatch);
        }

        let user = NewUser::parse(value.name, value.password).map_err(|e| match e {
            NewUserError::NameError(e) => RegistrationError::InvalidName(e),
            NewUserError::PasswordError(e) => RegistrationError::InvalidPassword(e),
        })?;

        Ok(user)
    }
}

pub async fn registration(
    _form: web::Form<RegistrationFormData>,
    _pool: web::Data<Pool>,
    _session: Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    todo!()
}
