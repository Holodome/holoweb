use crate::domain::users::Credentials;
use crate::middleware::Session;
use crate::services::{validate_credentials, AuthError};
use crate::utils::render_template;
use crate::utils::see_other;
use crate::Pool;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use secrecy::Secret;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    messages: IncomingFlashMessages,
}

#[tracing::instrument(skip(messages))]
pub async fn login_form(messages: IncomingFlashMessages) -> actix_web::Result<HttpResponse> {
    render_template(LoginTemplate { messages })
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials(_) => StatusCode::BAD_REQUEST,
            Self::AuthError(_) => StatusCode::BAD_REQUEST,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    password: Secret<String>,
}

#[tracing::instrument("Login", skip(form, pool, session))]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse, LoginError> {
    let credentials =
        Credentials::parse(form.0.name, form.0.password).map_err(LoginError::InvalidCredentials)?;
    // .map_err(|e| login_redirect(LoginError::InvalidCredentials(e)))?;

    match validate_credentials(credentials, &pool) {
        Ok(user_id) => {
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| LoginError::UnexpectedError(e.into()))?;
            // .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(see_other("/"))
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(e)
            // Err(login_redirect(e))
        }
    }
}

// fn login_redirect(e: LoginError) -> InternalError<LoginError> {
//     FlashMessage::error(e.to_string()).send();
//     InternalError::from_response(e, see_other("/login"))
// }
