use crate::domain::users::hashed_user_password::HashedUserPassword;
use crate::domain::users::{UserEmail, UserID, UserName, UserPasswordSalt};
use crate::schema::users;

#[derive(diesel::AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: UserID,
    pub name: Option<UserName>,
    pub email: Option<UserEmail>,
    pub password: Option<HashedUserPassword>,
    pub password_salt: Option<UserPasswordSalt>,
    pub is_banned: Option<bool>,
}
