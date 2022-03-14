use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub contents: String
}

#[derive(Debug)]
pub struct NewPost<'a> {
    pub name: &'a str,
    pub contents: &'a str
}