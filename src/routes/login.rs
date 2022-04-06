use crate::domain::credentials::Credentials;
use crate::domain::users::{NewUser, NewUserError, PasswordError, UserID};
use crate::middleware::require_login;
use crate::middleware::Session;
use crate::services::{get_user_by_name, insert_new_user, validate_credentials, AuthError};
use crate::startup::Pool;
use crate::utils::{extract_errors, extract_infos, see_other};
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{route, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use actix_web_lab::middleware::from_fn;
use askama::Template;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use std::fmt::Formatter;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    errors: Vec<String>,
    infos: Vec<String>,
    current_user_id: Option<UserID>,
}

#[tracing::instrument(skip(flash_messages, session))]
#[route("/login", method = "GET")]
pub async fn login_form(
    flash_messages: IncomingFlashMessages,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let user_id = session.get_user_id().unwrap();
    let s = LoginTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_id: user_id,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    password: Secret<String>,
}

#[tracing::instrument("Login", skip(form, pool, session))]
#[route("/login", method = "POST")]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials::parse(form.0.name, form.0.password)
        .map_err(|e| login_redirect(LoginError::InvalidCredentials(e)))?;

    match validate_credentials(credentials, &pool) {
        Ok(user_id) => {
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(see_other("/"))
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/login"))
}

#[route("/logout", method = "GET", wrap = "from_fn(require_login)")]
pub async fn logout(session: Session) -> HttpResponse {
    session.log_out();
    FlashMessage::info("You have successfully logged out").send();
    see_other("/login")
}

#[derive(Template)]
#[template(path = "registration.html")]
struct RegistrationTemplate {
    errors: Vec<String>,
    current_user_id: Option<UserID>,
}

#[tracing::instrument(skip(flash_messages, session))]
#[route("/registration", method = "GET")]
pub async fn registration_form(
    flash_messages: IncomingFlashMessages,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().unwrap();

    let s = RegistrationTemplate {
        errors: extract_errors(&flash_messages),
        current_user_id: user_id,
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
#[route("/registration", method = "POST")]
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
