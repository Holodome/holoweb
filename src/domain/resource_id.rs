use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::Write;
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, derive_more::Display, diesel::AsExpression, diesel::FromSqlRow,
)]
#[sql_type = "diesel::sql_types::Text"]
pub struct ResourceID {
    s: String,
}

// NOTE: We want to make serialization as is it were a single string without fields,
// but serde does not have an attribute to allow that. And we can't make ResourceID
// a tuple struct because diesel will not be happy about it.
// So we have to go with ways and implement derives ourselves.
impl Serialize for ResourceID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str(&self.s)?)
    }
}

impl<'de> Deserialize<'de> for ResourceID {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            s: String::deserialize(deserializer)?,
        })
    }
}

impl FromSql<diesel::sql_types::Text, Sqlite> for ResourceID {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)
            .map(|s| ResourceID { s })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for ResourceID {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(&self.s, out)
    }
}

impl ResourceID {
    pub fn generate_random() -> Self {
        Self {
            s: Uuid::new_v4().to_string(),
        }
    }
}

impl AsRef<String> for ResourceID {
    fn as_ref(&self) -> &String {
        &self.s
    }
}
