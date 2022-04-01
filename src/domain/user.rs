use crate::domain::{UserEmail, UserID, UserName, UserPassword};
use crate::schema::users;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct User {
    pub id: UserID,
    pub name: UserName,
    pub email: UserEmail,
    pub password: UserPassword,
    pub created_at: String,
    pub is_banned: bool,
}
