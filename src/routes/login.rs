use crate::domain::users::{Credentials, PasswordError, UserName, UserPassword};
use crate::middleware::Session;
use crate::services::{validate_credentials, AuthError};
use crate::utils::see_other;
use crate::utils::{redirect_with_error, render_template};
use crate::Pool;
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use secrecy::Secret;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    messages: IncomingFlashMessages,
    name: Option<&'a str>,
}

#[derive(serde::Deserialize)]
pub struct LoginQueryData {
    name: Option<String>,
}

#[tracing::instrument(skip(messages, query))]
pub async fn login_form(
    messages: IncomingFlashMessages,
    query: web::Query<LoginQueryData>,
) -> actix_web::Result<HttpResponse> {
    render_template(LoginTemplate {
        messages,
        name: query.0.name.as_deref(),
    })
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
    let login_redirect =
        |e| redirect_with_error(format!("/login?name={}", &form.0.name).as_str(), e);

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
