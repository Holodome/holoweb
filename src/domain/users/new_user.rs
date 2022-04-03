use crate::domain::users::{PasswordError, UserName, UserPassword};
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
        let name = UserName::parse(name).map_err(NewUserError::NameError)?;
        let password = UserPassword::parse(password).map_err(NewUserError::PasswordError)?;
        Ok(Self { name, password })
    }
}
