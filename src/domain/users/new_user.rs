use crate::domain::users::{UserName, UserPassword};

#[derive(Debug)]
pub struct NewUser {
    pub name: UserName,
    pub password: UserPassword,
}
