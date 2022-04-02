use crate::domain::users::hashed_user_password::HashedUserPassword;
use crate::domain::users::{UserEmail, UserID, UserName, UserPasswordSalt};
use crate::schema::users;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct User {
    pub id: UserID,
    pub name: UserName,
    pub email: UserEmail,
    pub password: HashedUserPassword,
    pub password_salt: UserPasswordSalt,
    pub created_at: String,
    pub is_banned: bool,
}