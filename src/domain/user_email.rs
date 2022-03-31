use std::io::Write;
use validator::validate_email;

#[derive(Debug, diesel::Queryable)]
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
