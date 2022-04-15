use crate::middleware::Session;
use actix_web::dev::Payload;
use actix_web::error::{ErrorBadRequest, ErrorForbidden};
use actix_web::{FromRequest, HttpRequest};
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use futures_util::future::{err, ok, Ready};
use std::io::Write;
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    PartialEq,
    derive_more::Display,
    diesel::AsExpression,
    diesel::FromSqlRow,
    serde::Deserialize,
    serde::Serialize,
)]
#[sql_type = "diesel::sql_types::Text"]
pub struct UserID {
    s: String,
}

impl FromSql<diesel::sql_types::Text, Sqlite> for UserID {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes).map(|s| UserID { s })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for UserID {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(&self.s, out)
    }
}

impl UserID {
    pub fn generate_random() -> Self {
        Self {
            s: Uuid::new_v4().to_string(),
        }
    }
}

impl AsRef<String> for UserID {
    fn as_ref(&self) -> &String {
        &self.s
    }
}

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
