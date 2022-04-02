use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sqlite::Sqlite;
use secrecy::{ExposeSecret, Secret};
use std::io::Write;

const PASSWORD_MIN_LENGTH: usize = 8;
const PASSWORD_MAX_LENGTH: usize = 128;

#[derive(thiserror::Error, Debug)]
pub enum PasswordError {
    #[error("Password is too short")]
    TooShort,
    #[error("Password is too long")]
    TooLong,
    #[error("Password must have both upper and lower case letters, as well as digits")]
    NoUpperAndLowerAndDigits,
    #[error("Invalid characters")]
    InvalidCharacters,
}

#[derive(Debug, Clone, diesel::AsExpression)]
#[sql_type = "diesel::sql_types::Text"]
pub struct UserPassword {
    s: Secret<String>,
}

impl UserPassword {
    pub fn parse(s: Secret<String>) -> Result<UserPassword, PasswordError> {
        {
            let s = s.expose_secret();
            if s.len() < PASSWORD_MIN_LENGTH {
                return Err(PasswordError::TooShort);
            }

            if s.len() > PASSWORD_MAX_LENGTH {
                return Err(PasswordError::TooLong);
            }

            if s.chars().any(|c| c.is_ascii_control()) || !s.is_ascii() {
                return Err(PasswordError::InvalidCharacters);
            }

            let has_lowercase = s.chars().any(|c| c.is_ascii_lowercase());
            let has_uppercase = s.chars().any(|c| c.is_ascii_uppercase());
            let has_digits = s.chars().any(|c| c.is_ascii_digit());
            if !has_digits || !has_uppercase || !has_lowercase {
                return Err(PasswordError::NoUpperAndLowerAndDigits);
            }
        }

        Ok(UserPassword { s })
    }
}

impl AsRef<Secret<String>> for UserPassword {
    fn as_ref(&self) -> &Secret<String> {
        &self.s
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::UserPassword;
    use claim::{assert_err, assert_ok};

    use secrecy::Secret;

    #[test]
    fn too_short_password_is_rejected() {
        let password = Secret::new("!1Aaaaa".to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn a_8_length_password_is_accepted() {
        let password = Secret::new("!1Aaaaaa".to_string());
        assert_ok!(UserPassword::parse(password));
    }

    #[test]
    fn too_long_password_is_rejected() {
        let mut password = "!1Aa".to_string();
        password.push_str("a".repeat(254).as_str());
        let password: Secret<String> = Secret::new(password);
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn non_ascii_characters_are_rejected() {
        let password = Secret::new("Пароль1234".to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn ascii_control_characters_are_rejected() {
        let password = Secret::new("\n\r123456".to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn only_lowercase_is_rejected() {
        let password = Secret::new("a".repeat(8));
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn only_uppercase_is_rejected() {
        let password = Secret::new("A".repeat(8));
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn only_digits_is_rejected() {
        let password = Secret::new("1".repeat(8));
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn password_without_digits_is_rejected() {
        let password = Secret::new("Aa".repeat(4));
        assert_err!(UserPassword::parse(password));
    }
}
