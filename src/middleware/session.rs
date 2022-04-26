use crate::domain::users::UserID;
use actix_session::SessionExt;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use secrecy::Secret;
use serde::de::DeserializeOwned;
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct Session(actix_session::Session);

impl Session {
    const USER_ID_KEY: &'static str = "user_id";
    const CSRF_TOKEN_KEY: &'static str = "csrf";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn get_csrf_token(&self) -> Result<Secret<String>, anyhow::Error> {
        let token = if let Some(token) = self.0.get(Self::CSRF_TOKEN_KEY)? {
            Secret::new(token)
        } else {
            let token = Uuid::new_v4().to_string();
            self.0.insert(Self::CSRF_TOKEN_KEY, token.clone())?;
            Secret::new(token)
        };

        Ok(token)
    }

    pub fn insert_user_id(&self, user_id: UserID) -> Result<(), anyhow::Error> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
            .map_err(|e| anyhow::anyhow!("Failed to insert user id: {:?}", e))
    }

    pub fn get_user_id(&self) -> Result<Option<UserID>, anyhow::Error> {
        let r = self.0.get(Self::USER_ID_KEY)?;
        Ok(r)
    }

    pub fn insert_form_data<D>(&self, key: &str, form_data: D) -> Result<(), anyhow::Error>
    where
        D: serde::Serialize,
    {
        self.0
            .insert(key, form_data)
            .map_err(|e| anyhow::anyhow!("Failed to insert form data '{:?}': {:?}", key, e))
    }

    pub fn pop_form_data<D>(&self, key: &str) -> Result<Option<D>, anyhow::Error>
    where
        D: DeserializeOwned,
    {
        let r = self.0.get::<D>(key)?;
        self.0.remove(key);
        Ok(r)
    }

    pub fn log_out(self) {
        self.0.purge();
    }

    pub fn from_request_sync(req: &HttpRequest) -> Self {
        Session(req.get_session())
    }
}

impl FromRequest for Session {
    type Error = <actix_session::Session as FromRequest>::Error;
    type Future = Ready<Result<Session, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(Session::from_request_sync(req)))
    }
}
