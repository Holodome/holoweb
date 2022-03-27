use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct UserName(String);

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

        Ok(Self(s))
    }
}

impl AsRef<String> for UserName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user_name::UserName;
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
