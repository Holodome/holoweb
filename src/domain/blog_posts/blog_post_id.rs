use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use serde::{Deserialize, Deserializer};
use std::io::Write;
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, derive_more::Display, diesel::AsExpression, diesel::FromSqlRow,
)]
#[sql_type = "diesel::sql_types::Text"]
pub struct BlogPostID {
    s: String,
}

impl<'de> Deserialize<'de> for BlogPostID {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            s: String::deserialize(deserializer)?,
        })
    }
}

impl FromSql<diesel::sql_types::Text, Pg> for BlogPostID {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Pg>>::from_sql(bytes).map(|s| BlogPostID { s })
    }
}

impl ToSql<diesel::sql_types::Text, Pg> for BlogPostID {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Pg>>::to_sql(&self.s, out)
    }
}

impl BlogPostID {
    pub fn generate_random() -> Self {
        Self {
            s: Uuid::new_v4().to_string(),
        }
    }
}

impl AsRef<String> for BlogPostID {
    fn as_ref(&self) -> &String {
        &self.s
    }
}
