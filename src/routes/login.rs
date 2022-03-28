use std::fmt::Formatter;
use actix_web::{HttpResponse, web};
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama::Template;
use secrecy::Secret;
use crate::authentication::{AuthError, Credentials, validate_credentials};
use crate::session::Session;
use crate::startup::Pool;
use crate::utils::see_other;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    errors: Vec<String>
}

#[tracing::instrument(
    skip(flash_messages)
)]
pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let errors = flash_messages.iter()
        .map(|m| m.content().to_string()).collect::<Vec<_>>();
    let s = LoginTemplate {
        errors
    }.render().unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(s)
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error)
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
    password: Secret<String>
}

pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    session: Session
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.name,
        password: form.0.password
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(
                see_other("/home")
            )
        },
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into())
            };
            Err(login_redirect(e))
        }
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/login"))
}