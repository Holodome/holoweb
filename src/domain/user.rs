use crate::domain::{UserEmail, UserName, UserPassword};



#[derive(Debug, diesel::Queryable)]
pub struct User {
    id: String,
    name: UserName,
    email: UserEmail,
    password: UserPassword,
    created_at: String,
    is_banned: bool,
}
