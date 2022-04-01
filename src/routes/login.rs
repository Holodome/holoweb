use crate::domain::Credentials;
use crate::services::{validate_credentials, AuthError};
use crate::session::Session;
use crate::startup::Pool;
use crate::utils::see_other;
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama::Template;
use secrecy::Secret;
use serde::Deserialize;
use std::fmt::Formatter;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    errors: Vec<String>,
}

#[tracing::instrument(skip(flash_messages))]
pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let errors = flash_messages
        .iter()
        .map(|m| m.content().to_string())
        .collect::<Vec<_>>();
    let s = LoginTemplate { errors }.render().unwrap();
    HttpResponse::Ok().content_type(ContentType::html()).body(s)
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
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials::parse(form.0.name, form.0.password)
        .map_err(|e| login_redirect(LoginError::InvalidCredentials(e)))?;

    tracing::Span::current().record("user_name", &tracing::field::display(&credentials.name));
    match validate_credentials(credentials, &pool).await {
        Ok(user_name) => {
            tracing::Span::current().record("user_name", &tracing::field::display(&user_name));
            session.renew();
            session
                .insert_user_name(user_name)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(see_other("/home"))
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
