use crate::domain::users::{UserID, UserName};
use actix_session::SessionExt;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::future::{ready, Ready};

pub struct Session(actix_session::Session);

impl Session {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: UserID) -> Result<(), serde_json::Error> {
        self.0
            .insert(Self::USER_ID_KEY, user_id.as_ref().to_string())
    }

    pub fn get_user_id(&self) -> Result<Option<UserID>, serde_json::Error> {
        Ok(self
            .0
            .get(Self::USER_ID_KEY)?
            .map(|name| UserID::new(name)))
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
