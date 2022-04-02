use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use std::io::Write;
use unicode_segmentation::UnicodeSegmentation;

#[derive(
    Debug, Clone, PartialEq, derive_more::Display, diesel::AsExpression, diesel::FromSqlRow,
)]
#[sql_type = "diesel::sql_types::Text"]
pub struct UserName {
    s: String,
}

impl UserName {
    pub fn parse(s: String) -> Result<UserName, anyhow::Error> {
        if s.trim().is_empty() {
            return Err(anyhow::anyhow!("{} user name is whitespace or empty", s));
        }

        if s.graphemes(true).count() > 256 {
            return Err(anyhow::anyhow!("{} user name is too long", s));
        }

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        if s.chars().any(|g| forbidden_characters.contains(&g)) {
            return Err(anyhow::anyhow!(
                "{} user name contains forbidden characters",
                s
            ));
        }

        Ok(Self { s })
    }
}

impl FromSql<diesel::sql_types::Text, Sqlite> for UserName {
    fn from_sql(
        bytes: Option<&<Sqlite as Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)
            .map(|s| UserName { s })
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for UserName {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> diesel::serialize::Result {
        <String as ToSql<diesel::sql_types::Text, Sqlite>>::to_sql(&self.s, out)
    }
}

impl AsRef<String> for UserName {
    fn as_ref(&self) -> &String {
        &self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::Username;
    use fake::Fake;
    use quickcheck::Gen;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(UserName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        let result = UserName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        let result = UserName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn emtpy_string_is_rejected() {
        let name = "".to_string();
        let result = UserName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn names_containing_invalid_characters_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            let result = UserName::parse(name);
            assert_err!(result);
        }
    }

    #[derive(Debug, Clone)]
    struct ValidNameFixture(pub String);

    impl quickcheck::Arbitrary for ValidNameFixture {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let name = Username().fake_with_rng(g);
            Self(name)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_name_is_valid(valid_name: ValidNameFixture) -> bool {
        UserName::parse(valid_name.0).is_ok()
    }
}
