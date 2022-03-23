use crate::{HmacSecret, Pool, StatusCode};
use actix_web::body::BoxBody;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse, ResponseError};
use askama::filters::format;
use askama::Template;
use derive_more::Display;
use hmac::{Hmac, Mac};
use log::Log;
use secrecy::{ExposeSecret, Secret};
use std::fmt::{Display, Formatter};

#[derive(Debug, Display)]
pub enum LoginError {
    #[display(fmt = "Invalid name")]
    InvalidName,
    #[display(fmt = "Invalid password")]
    InvalidPassword,
    #[display(fmt = "Unexpected")]
    Unexpected,
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::InvalidName => StatusCode::UNAUTHORIZED,
            LoginError::InvalidPassword => StatusCode::UNAUTHORIZED,
            LoginError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

pub type Result<T, E = LoginError> = std::result::Result<T, E>;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    password: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;
        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));

        let mut mac =
            Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes()).unwrap();

        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct PageTemplate {
    error: Option<String>,
}

// pub async fn login_post(form: web::Form<FormData>, pool: web::Data<Pool>)
//     -> Result<HttpResponse> {
//
// }

pub async fn login_get(
    query: Option<web::Query<QueryParams>>,
    secret: web::Data<HmacSecret>,
) -> HttpResponse {
    let error = match query {
        None => None,
        Some(query) => match query.0.verify(&secret) {
            Ok(error) => Some(htmlescape::encode_minimal(&error)),
            Err(e) => {
                log::warn!("Failed to verify query parameters using hmac tag ({})", e);
                None
            }
        },
    };
    let s = PageTemplate { error }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}
