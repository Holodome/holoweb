use chrono::{Duration, Utc};
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
        let now = Self::now();
        self.since(&now)
    }

    pub fn since(&self, now: &DateTime) -> String {
        let difference = now.t - self.t;
        duration_since_human_readable(difference)
    }
}

fn duration_since_human_readable(diff: Duration) -> String {
    let weeks = diff.num_weeks();
    if weeks != 0 {
        return if weeks != 1 {
            format!("{} weeks ago", weeks)
        } else {
            "1 week ago".to_string()
        };
    }

    let days = diff.num_days();
    if days != 0 {
        return if days != 1 {
            format!("{} days ago", days)
        } else {
            "1 day ago".to_string()
        };
    }

    let hours = diff.num_hours();
    if hours != 0 {
        return if hours != 1 {
            format!("{} hours ago", hours)
        } else {
            "1 hour ago".to_string()
        };
    }

    let minutes = diff.num_minutes();
    if minutes != 0 {
        return if minutes != 1 {
            format!("{} minutes ago", minutes)
        } else {
            "1 minute ago".to_string()
        };
    }

    let seconds = diff.num_seconds();
    if seconds != 0 {
        return if seconds != 1 {
            format!("{} seconds ago", seconds)
        } else {
            "1 second ago".to_string()
        };
    }

    "just now".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_week() {
        let duration = Duration::weeks(1);
        assert_eq!(duration_since_human_readable(duration), "1 week ago");
    }

    #[test]
    fn test_many_weeks() {
        let duration = Duration::weeks(123123);
        assert_eq!(duration_since_human_readable(duration), "123123 weeks ago");
    }

    #[test]
    fn test_one_day() {
        let duration = Duration::days(1);
        assert_eq!(duration_since_human_readable(duration), "1 day ago");
    }

    #[test]
    fn test_multiple_days() {
        let duration = Duration::days(5);
        assert_eq!(duration_since_human_readable(duration), "5 days ago");
    }

    #[test]
    fn test_one_hour_works() {
        let duration = Duration::hours(1);
        assert_eq!(duration_since_human_readable(duration), "1 hour ago");
    }

    #[test]
    fn test_multiple_hours() {
        let duration = Duration::hours(5);
        assert_eq!(duration_since_human_readable(duration), "5 hours ago");
    }

    #[test]
    fn test_minute_below_day() {
        let duration = Duration::hours(23) + Duration::minutes(59);
        assert_eq!(duration_since_human_readable(duration), "23 hours ago");
    }

    #[test]
    fn test_one_minute_works() {
        let duration = Duration::minutes(1);
        assert_eq!(duration_since_human_readable(duration), "1 minute ago");
    }

    #[test]
    fn test_one_second_works() {
        let duration = Duration::seconds(1);
        assert_eq!(duration_since_human_readable(duration), "1 second ago");
    }

    #[test]
    fn test_just_now() {
        let duration = Duration::seconds(0);
        assert_eq!(duration_since_human_readable(duration), "just now");
    }
}
