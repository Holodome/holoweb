use std::fmt::Formatter;
use actix_web::{HttpResponse, web};
use actix_web::error::InternalError;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use actix_web::http::header::ContentType;
use secrecy::Secret;
use crate::routes::LoginError;
use crate::session::Session;
use crate::startup::Pool;

#[derive(Template)]
#[template(path = "registration.html")]
struct RegistrationTemplate {
    errors: Vec<String>
}

#[tracing::instrument(skip(flash_messages))]
pub async fn registration_form(flash_messages: IncomingFlashMessages)
    -> HttpResponse {
    let errors: Vec<String> = flash_messages
        .iter()
        .map(|m| m.content().to_string())
        .collect();
    let s = RegistrationTemplate { errors }.render().unwrap();
    HttpResponse::Ok().content_type(ContentType::html()).body(s)
}

#[derive(thiserror::Error)]
pub enum RegistrationError {
    #[error("Name is already taken")]
    TakenName(#[source] anyhow::Error),
    #[error("Unsafe password")]
    UnsafePassword(#[source] anyhow::Error),
    #[error("Passwords don't match")]
    PasswordsDontMatch(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error)
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
    repeat_password: Secret<String>
}

pub async fn registration(
    form: web::Form<RegistrationFormData>,
    pool: web::Data<Pool>,
    session: Session
) -> Result<HttpResponse, InternalError<LoginError>> {
    todo!()
}