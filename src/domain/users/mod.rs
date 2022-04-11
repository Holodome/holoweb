pub mod hashed_user_password;
mod new_user;
mod stored_credentials;
mod update_user;
mod user;
mod user_email;
mod user_name;
mod user_password;
mod user_password_salt;

pub type UserID = ResourceID;

impl FromRequest for UserID {
    type Error = <actix_session::Session as FromRequest>::Error;
    type Future = Ready<Result<UserID, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = Session::from_request_sync(req);
        match session.get_user_id() {
            Ok(id) => match id {
                Some(id) => ok(id),
                None => err(ErrorForbidden("User is not authenticated")),
            },
            Err(e) => err(ErrorBadRequest(e)),
        }
    }
}

use crate::domain::resource_id::ResourceID;
use crate::middleware::Session;
use actix_web::dev::Payload;
use actix_web::error::{ErrorBadRequest, ErrorForbidden};
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
pub use new_user::*;
pub use stored_credentials::*;
pub use update_user::*;
pub use user::*;
pub use user_email::*;
pub use user_name::*;
pub use user_password::*;
pub use user_password_salt::*;
