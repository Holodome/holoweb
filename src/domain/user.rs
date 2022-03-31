use crate::domain::{UserEmail, UserName, UserPassword};
use diesel::Queryable;
use secrecy::Secret;

#[derive(Debug, diesel::Queryable)]
pub struct User {
    id: String,
    name: UserName,
    email: UserEmail,
    password: UserPassword,
    created_at: String,
    is_banned: bool,
}
