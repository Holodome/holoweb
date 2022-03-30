use secrecy::{ExposeSecret, Secret};

const PASSWORD_MIN_LENGTH: usize = 8;
const PASSWORD_MAX_LENGTH: usize = 128;

#[derive(thiserror::Error, Debug)]
enum PasswordError {
    #[error("Password is too short")]
    TooShort,
    #[error("Password is too long")]
    TooLong,
    #[error("Password must have both upper and lower case letters, as well as digits")]
    NoUpperAndLowerAndDigits,
    #[error("Invalid characters")]
    InvalidCharacters
}

pub struct UserPassword(Secret<String>);

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

        Ok(UserPassword(s))
    }
}