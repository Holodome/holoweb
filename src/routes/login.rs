use crate::domain::users::{Credentials, PasswordError, UserName, UserPassword};
use crate::middleware::Session;
use crate::services::{validate_credentials, AuthError};
use crate::utils::{e500, see_other};
use crate::utils::{redirect_with_error, render_template};
use crate::Pool;
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use secrecy::Secret;

const LOGIN_FORM_SESSION_KEY: &str = "login_form";

#[derive(serde::Serialize, serde::Deserialize)]
struct LoginCache {
    name: String,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    messages: IncomingFlashMessages,
    name: Option<&'a str>,
}

#[tracing::instrument(skip(messages, session))]
pub async fn login_form(
    messages: IncomingFlashMessages,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let form_data = session
        .pop_form_data::<LoginCache>(LOGIN_FORM_SESSION_KEY)
        .map_err(e500)?;

    let name = form_data.as_ref().map(|f| f.name.as_str());

    render_template(LoginTemplate { messages, name })
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Invalid name")]
    InvalidName(#[source] anyhow::Error),
    #[error("Invalid password")]
    InvalidPassword(#[source] PasswordError),
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;
        error_chain_fmt(self, f)
    }
}

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    name: String,
    password: Secret<String>,
}

#[tracing::instrument("Login", skip(form, pool, session))]
pub async fn login(
    form: web::Form<LoginFormData>,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let login_redirect = |e| {
        let e = if let Err(new_e) = session.insert_form_data(
            LOGIN_FORM_SESSION_KEY,
            LoginCache {
                name: form.0.name.clone(),
            },
        ) {
            LoginError::UnexpectedError(anyhow::anyhow!(
                "Failed to execute request: {:?} & failed to cache data: {:?}",
                e,
                new_e
            ))
        } else {
            e
        };
        redirect_with_error("/login", e)
    };

    let credentials = Credentials {
        name: UserName::parse(&form.0.name)
            .map_err(LoginError::InvalidName)
            .map_err(login_redirect)?,
        password: UserPassword::parse(form.0.password)
            .map_err(LoginError::InvalidPassword)
            .map_err(login_redirect)?,
    };

    match validate_credentials(credentials, &pool) {
        Ok(user_id) => {
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(LoginError::UnexpectedError)
                .map_err(login_redirect)?;
            Ok(see_other("/blog_posts/all"))
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
