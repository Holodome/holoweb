use crate::domain::{HashedUserPassword, UserName, UserPasswordSalt};

#[derive(Debug, Clone, diesel::Queryable)]
pub struct StoredCredentials {
    pub name: UserName,
    pub password: HashedUserPassword,
    pub salt: UserPasswordSalt
}
