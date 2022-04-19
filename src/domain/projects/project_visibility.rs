use anyhow::anyhow;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use std::io::Write;

const ALL: &str = "all";
const AUTHENTICATED: &str = "authenticated";

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
pub enum ProjectVisibility {
    All,
    Authenticated,
}

impl FromSql<diesel::sql_types::Text, Sqlite> for ProjectVisibility {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes).and_then(|s| {
            Ok(match s.as_str() {
                ALL => Ok(ProjectVisibility::All),
                AUTHENTICATED => Ok(ProjectVisibility::Authenticated),
                _ => Err(anyhow!("{} is not a valid project visibility", s)),
            }?)
        })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for ProjectVisibility {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        let s = match self {
            ProjectVisibility::All => ALL.to_string(),
            ProjectVisibility::Authenticated => AUTHENTICATED.to_string(),
        };
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(&s, out)
    }
}
