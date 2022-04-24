use chrono::Utc;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use std::io::Write;
use std::str::FromStr;

type Inner = chrono::DateTime<chrono::Utc>;

#[derive(
    Debug, Clone, PartialEq, derive_more::Display, diesel::AsExpression, diesel::FromSqlRow,
)]
#[sql_type = "diesel::sql_types::Text"]
pub struct DateTime {
    t: Inner,
}

impl FromSql<diesel::sql_types::Text, Sqlite> for DateTime {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)
            .and_then(|s| {
                chrono::DateTime::<chrono::Utc>::from_str(&s)
                    .map_err(anyhow::Error::new)
                    .map_err(|e| e.into())
            })
            .map(|t| DateTime { t })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for DateTime {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(&self.t.to_string(), out)
    }
}

impl AsRef<chrono::DateTime<chrono::Utc>> for DateTime {
    fn as_ref(&self) -> &chrono::DateTime<Utc> {
        &self.t
    }
}

impl DateTime {
    pub fn now() -> Self {
        Self {
            t: chrono::Utc::now(),
        }
    }

    pub fn ago(&self) -> String {
        let now = chrono::Utc::now();
        self.since(now)
    }

    pub fn since(&self, now: chrono::DateTime<Utc>) -> String {
        let difference = now - self.t;

        let weeks = difference.num_weeks();
        if weeks != 0 {
            return format!("{} weeks ago", weeks);
        }

        let days = difference.num_days();
        if days != 0 {
            return format!("{} days ago", days);
        }

        let hours = difference.num_hours();
        if hours != 0 {
            return format!("{} hours ago", hours);
        }

        let minutes = difference.num_minutes();
        if minutes != 0 {
            return format!("{} minutes ago", minutes);
        }

        let seconds = difference.num_seconds();
        if seconds != 0 {
            return format!("{} seconds ago", seconds);
        }

        "just now".to_string()
    }
}
