use crate::domain::users::hashed_user_password::HashedUserPassword;
use crate::domain::users::{UserID, UserName, UserPasswordSalt};

#[derive(Debug, Clone, diesel::Queryable)]
pub struct StoredCredentials {
    pub name: UserName,
    pub password: HashedUserPassword,
    pub salt: UserPasswordSalt,
    pub user_id: UserID
}
