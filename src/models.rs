use serde::{Serialize, Deserialize};
use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct Post {
    pub id: String,
    pub name: String,
    pub contents: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub name: String,
    pub contents: String
}