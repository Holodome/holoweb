use crate::schema::users;

#[derive(Debug, serde::Serialize, serde::Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub role: String,
    pub is_banned: bool,
}

#[derive(Debug)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: &'a str,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub id: &'a str,
    pub name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
    pub created_at: Option<&'a str>,
    pub role: Option<&'a str>,
    pub is_banned: Option<bool>,
}
