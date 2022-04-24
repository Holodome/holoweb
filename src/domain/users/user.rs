use crate::domain::time::DateTime;
use crate::domain::users::hashed_user_password::HashedUserPassword;
use crate::domain::users::{UserEmail, UserID, UserName, UserPasswordSalt, UserRole};
use crate::schema::users;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct User {
    pub id: UserID,
    pub name: UserName,
    pub email: UserEmail,

    pub created_at: DateTime,

    pub password: HashedUserPassword,
    pub password_salt: UserPasswordSalt,

    pub is_banned: bool,
    pub role: UserRole,
}
