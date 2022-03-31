use uuid::Uuid;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use std::io::Write;

#[derive(Debug, derive_more::Display, diesel::AsExpression, diesel::FromSqlRow)]
#[sql_type = "diesel::sql_types::Text"]
pub struct UserID {
    s: String,
}

impl FromSql<diesel::sql_types::Text, Sqlite> for UserID {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)
            .map(|s| UserID { s })
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
