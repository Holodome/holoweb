use crate::domain::user_email::UserEmail;
use crate::domain::user_name::UserName;
use crate::domain::user_password::PasswordError;
use crate::domain::UserPassword;
use secrecy::Secret;

#[derive(thiserror::Error, Debug)]
pub enum NewUserError {
    #[error("Invalid name")]
    NameError(#[source] anyhow::Error),
    #[error("Invalid password")]
    PasswordError(#[source] PasswordError),
}

#[derive(Debug)]
pub struct NewUser {
    pub name: UserName,
    pub password: UserPassword,
}

impl NewUser {
    pub fn parse(name: String, password: Secret<String>) -> Result<Self, NewUserError> {
        let name = UserName::parse(name).map_err(|e| NewUserError::NameError(e))?;
        let password = UserPassword::parse(password).map_err(|e| NewUserError::PasswordError(e))?;
        Ok(Self { name, password })
    }
}
