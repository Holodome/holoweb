use serde::{Serialize, Deserialize};
use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub contents: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub name: &'a str,
    pub contents: &'a str
}