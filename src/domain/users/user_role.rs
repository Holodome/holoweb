use anyhow::anyhow;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use std::io::Write;

#[derive(
    Debug,
    Clone,
    PartialEq,
    derive_more::Display,
    diesel::AsExpression,
    diesel::FromSqlRow,
    serde::Serialize,
)]
#[sql_type = "diesel::sql_types::Text"]
pub enum UserRole {
    Admin,
    User,
}

impl FromSql<diesel::sql_types::Text, Sqlite> for UserRole {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes).and_then(|s| {
            Ok(match s.as_str() {
                "admin" => Ok(UserRole::Admin),
                "user" => Ok(UserRole::User),
                _ => Err(anyhow!("{} is not a valid user role", s)),
            }?)
        })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for UserRole {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        let s = match self {
            UserRole::Admin => "admin".to_string(),
            UserRole::User => "user".to_string(),
        };
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(&s, out)
    }
}
