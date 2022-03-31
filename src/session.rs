use actix_session::SessionExt;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::future::{ready, Ready};
use crate::domain::UserName;

pub struct Session(actix_session::Session);

impl Session {
    const USER_NAME_KEY: &'static str = "user_name";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_name(&self, user_name: UserName) -> Result<(), serde_json::Error> {
        self.0.insert(Self::USER_NAME_KEY, user_name.as_ref().to_string())
    }

    pub fn get_user_name(&self) -> Result<Option<String>, serde_json::Error> {
        self.0.get(Self::USER_NAME_KEY)
    }

    pub fn log_out(self) {
        self.0.purge();
    }
}

impl FromRequest for Session {
    type Error = <actix_session::Session as FromRequest>::Error;
    type Future = Ready<Result<Session, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(Session(req.get_session())))
    }
}
