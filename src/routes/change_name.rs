use crate::domain::credentials::Credentials;
use crate::domain::users::{UserID, UserName, UserPassword};
use crate::middleware::{require_login, Session};
use crate::services::{get_user_by_id, get_user_by_name, validate_credentials, AuthError};
use crate::startup::Pool;
use crate::utils::{extract_errors, extract_infos, see_other};
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{route, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use actix_web_lab::middleware::from_fn;
use askama::Template;
use secrecy::Secret;
use std::fmt::Formatter;

#[derive(Template)]
#[template(path = "change_name.html")]
struct PageTemplate {
    errors: Vec<String>,
    infos: Vec<String>,
    current_user_id: Option<UserID>,
}

#[route("/change_name", method = "GET", wrap = "from_fn(require_login)")]
pub async fn change_name_form(
    flash_messages: IncomingFlashMessages,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let user_id = session.get_user_id().unwrap().unwrap();
    let s = PageTemplate {
        errors: extract_errors(&flash_messages),
        infos: extract_infos(&flash_messages),
        current_user_id: Some(user_id),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_name: String,
}

#[derive(thiserror::Error)]
pub enum ChangeNameError {
    #[error("Current password is invalid")]
    InvalidCurrentPassword(#[source] anyhow::Error),
    #[error("Taken name")]
    TakenName,
    #[error("Invalid name")]
    InvalidName(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ChangeNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

#[route("/change_name", method = "POST", wrap = "from_fn(require_login)")]
#[tracing::instrument(skip(form, pool, session))]
pub async fn change_name(
    form: web::Form<FormData>,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse, InternalError<ChangeNameError>> {
    let user_id = session.get_user_id().unwrap().unwrap();

    let user_name = get_user_by_id(&pool, &user_id)
        .map_err(|e| redirect(ChangeNameError::UnexpectedError(e)))?
        .unwrap()
        .name;
    let credentials = Credentials {
        name: user_name.clone(),
        password: UserPassword::parse(form.current_password.clone())
            .map_err(|e| redirect(ChangeNameError::InvalidCurrentPassword(e.into())))?,
    };
    if let Err(e) = validate_credentials(credentials, &pool) {
        let e = match e {
            AuthError::InvalidCredentials(_) => ChangeNameError::InvalidCurrentPassword(e.into()),
            AuthError::UnexpectedError(_) => ChangeNameError::UnexpectedError(e.into()),
        };
        return Err(redirect(e));
    }

    let new_user_name = UserName::parse(form.new_name.clone())
        .map_err(|e| redirect(ChangeNameError::InvalidName(e)))?;
    if get_user_by_name(&pool, &new_user_name)
        .map_err(|e| redirect(ChangeNameError::UnexpectedError(e)))?
        .is_some()
    {
        return Err(redirect(ChangeNameError::TakenName));
    }

    crate::services::change_name(&pool, &user_name, &new_user_name)
        .map_err(|e| redirect(ChangeNameError::UnexpectedError(e)))?;

    FlashMessage::info("Your name has been changed").send();
    Ok(see_other("/account"))
}

fn redirect(e: ChangeNameError) -> InternalError<ChangeNameError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/change_name"))
}
