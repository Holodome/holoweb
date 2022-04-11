use crate::domain::users::hashed_user_password::HashedUserPassword;
use crate::domain::users::{UserEmail, UserID, UserName};
use crate::schema::users;

#[derive(diesel::AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub id: &'a UserID,
    pub name: Option<&'a UserName>,
    pub email: Option<&'a UserEmail>,
    pub password: Option<&'a HashedUserPassword>,
    pub is_banned: Option<bool>,
}
