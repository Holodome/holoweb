use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use secrecy::{ExposeSecret, Secret};
use std::io::Write;
use uuid::Uuid;

#[derive(Debug, Clone, diesel::AsExpression)]
#[sql_type = "diesel::sql_types::Text"]
pub struct UserPasswordSalt {
    s: Secret<String>
}

impl UserPasswordSalt {
    pub fn generate_random() -> Self {
        Self {
            s: Secret::new(Uuid::new_v4().to_string())
        }
    }
}

impl PartialEq<UserPasswordSalt> for UserPasswordSalt {
    fn eq(&self, other: &UserPasswordSalt) -> bool {
        self.as_ref()
            .expose_secret()
            .eq(other.as_ref().expose_secret())
    }
}

impl diesel::Queryable<diesel::sql_types::Text, diesel::sqlite::Sqlite> for UserPasswordSalt {
    type Row = <String as diesel::Queryable<diesel::sql_types::Text, diesel::sqlite::Sqlite>>::Row;

    fn build(row: Self::Row) -> Self {
        UserPasswordSalt {
            s: Secret::new(row),
        }
    }
}

impl FromSql<diesel::sql_types::Text, Sqlite> for UserPasswordSalt {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)
            .map(|s| UserPasswordSalt { s: Secret::new(s) })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for UserPasswordSalt {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(self.s.expose_secret(), out)
    }
}

impl AsRef<Secret<String>> for UserPasswordSalt {
    fn as_ref(&self) -> &Secret<String> {
        &self.s
    }
}