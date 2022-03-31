use crate::domain::{UserName, UserPassword};
use secrecy::Secret;

#[derive(Debug, Clone, diesel::Queryable)]
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