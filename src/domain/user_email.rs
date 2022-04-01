use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use std::io::Write;
use validator::validate_email;

#[derive(Debug, Clone, PartialEq, derive_more::Display, diesel::AsExpression, diesel::FromSqlRow)]
#[sql_type = "diesel::sql_types::Text"]
pub struct UserEmail {
    s: String,
}

impl UserEmail {
    pub fn parse(s: String) -> Result<Self, anyhow::Error> {
        if validate_email(&s) {
            Ok(Self { s })
        } else {
            Err(anyhow::anyhow!("{} is not a valid user email", s))
        }
    }
}

impl FromSql<diesel::sql_types::Text, Sqlite> for UserEmail {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)
            .map(|s| UserEmail { s })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for UserEmail {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(&self.s, out)
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.s
    }
}

#[cfg(test)]
mod tests {
    use super::UserEmail;
    use claim::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(UserEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "useremail.com".to_string();
        assert_err!(UserEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@useremail.com".to_string();
        assert_err!(UserEmail::parse(email));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        UserEmail::parse(valid_email.0).is_ok()
    }
}
