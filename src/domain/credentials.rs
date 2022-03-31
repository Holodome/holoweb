use crate::domain::{UserName, UserPassword};

#[derive(Debug, diesel::Queryable)]
pub struct Credentials {
    pub user_name: UserName,
    pub user_password: UserPassword
}
