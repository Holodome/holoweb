use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Deserializer};
use std::io::Write;
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, derive_more::Display, diesel::AsExpression, diesel::FromSqlRow,
)]
#[sql_type = "diesel::sql_types::Text"]
pub struct ProjectID {
    s: String,
}

impl<'de> Deserialize<'de> for ProjectID {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            s: String::deserialize(deserializer)?,
        })
    }
}

impl FromSql<diesel::sql_types::Text, Sqlite> for ProjectID {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)
            .map(|s| ProjectID { s })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for ProjectID {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(&self.s, out)
    }
}

impl ProjectID {
    pub fn generate_random() -> Self {
        Self {
            s: Uuid::new_v4().to_string(),
        }
    }
}

impl AsRef<String> for ProjectID {
    fn as_ref(&self) -> &String {
        &self.s
    }
}
