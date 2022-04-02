use crate::domain::{HashedUserPassword, UserName};

#[derive(Debug, Clone, diesel::Queryable)]
pub struct StoredCredentials {
    pub name: UserName,
    pub password: HashedUserPassword,
}
