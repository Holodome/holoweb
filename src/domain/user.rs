use crate::domain::{UserEmail, UserName, UserPassword, UserID};

#[derive(Debug, diesel::Queryable)]
pub struct User {
    id: UserID,
    name: UserName,
    email: UserEmail,
    password: UserPassword,
    created_at: String,
    is_banned: bool,
}
