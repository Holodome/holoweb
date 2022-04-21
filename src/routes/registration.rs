use crate::domain::users::{NewUser, PasswordError, UserName, UserPassword};
use crate::middleware::Session;
use crate::services::{insert_new_user, UserError};
use crate::utils::{e500, see_other};
use crate::utils::{redirect_with_error, render_template};
use crate::Pool;
use actix_web::error::InternalError;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use secrecy::{ExposeSecret, Secret};
use std::fmt::Formatter;

const REGISTRATION_FORM_SESSION_KEY: &str = "registration_form";

#[derive(serde::Serialize, serde::Deserialize)]
struct RegistrationCache {
    name: String,
}

#[derive(Template)]
#[template(path = "registration.html")]
struct RegistrationTemplate<'a> {
    messages: IncomingFlashMessages,
    name: Option<&'a str>,
}

#[tracing::instrument(skip(messages, session))]
pub async fn registration_form(
    messages: IncomingFlashMessages,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let form_data = session
        .pop_form_data::<RegistrationCache>(REGISTRATION_FORM_SESSION_KEY)
        .map_err(e500)?;

    let name = form_data.as_ref().map(|f| f.name.as_str());

    render_template(RegistrationTemplate { messages, name })
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

#[tracing::instrument("Registration", skip(form, pool, session))]
pub async fn registration(
    form: web::Form<RegistrationFormData>,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse, InternalError<RegistrationError>> {
    let registration_redirect = |e| {
        let e = if let Err(new_e) = session.insert_form_data(
            REGISTRATION_FORM_SESSION_KEY,
            RegistrationCache {
                name: form.0.name.clone(),
            },
        ) {
            RegistrationError::UnexpectedError(anyhow::anyhow!(
                "Failed to execute request: {:?} & failed to cache data: {:?}",
                e,
                new_e
            ))
        } else {
            e
        };
        redirect_with_error("/registration", e)
    };

    if form.0.password.expose_secret() != form.0.repeat_password.expose_secret() {
        return Err(registration_redirect(RegistrationError::PasswordsDontMatch));
    }

    let new_user = NewUser {
        name: UserName::parse(&form.0.name)
            .map_err(RegistrationError::InvalidName)
            .map_err(registration_redirect)?,
        password: UserPassword::parse(form.0.password)
            .map_err(RegistrationError::InvalidPassword)
            .map_err(registration_redirect)?,
    };

    match insert_new_user(&pool, &new_user) {
        Ok(user) => {
            session.renew();
            session
                .insert_user_id(user.id)
                .map_err(RegistrationError::UnexpectedError)
                .map_err(registration_redirect)?;
            Ok(see_other("/blog_posts/all"))
        }
        Err(e) => match e {
            UserError::TakenName => Err(registration_redirect(RegistrationError::TakenName)),
            _ => Err(registration_redirect(RegistrationError::UnexpectedError(
                e.into(),
            ))),
        },
    }
}
