use crate::domain::users::hashed_user_password::HashedUserPassword;
use crate::domain::users::{UserID, UserName, UserPassword, UserPasswordSalt};
use secrecy::Secret;

#[derive(Debug, Clone, diesel::Queryable)]
pub struct StoredCredentials {
    pub name: UserName,
    pub password: HashedUserPassword,
    pub salt: UserPasswordSalt,
    pub user_id: UserID,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub name: UserName,
    pub password: UserPassword,
}

impl Credentials {
    pub fn parse(name: String, password: Secret<String>) -> Result<Credentials, anyhow::Error> {
        let name = UserName::parse(name)?;
        let password = UserPassword::parse(password)?;
        Ok(Self { name, password })
    }
}
