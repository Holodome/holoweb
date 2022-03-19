use serde::{Serialize, Deserialize};
use crate::schema::{users, blog_posts, comments, projects};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub role: String,
    pub is_banned: bool
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: &'a str,
    pub role: &'a str,
    pub is_banned: bool
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub brief: Option<String>,
    pub contents: String,
    pub author_id: String
}

#[derive(Debug, Insertable)]
#[table_name = "blog_posts"]
pub struct NewBlogPost<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub brief: Option<&'a str>,
    pub contents: &'a str,
    pub author_id: &'a str
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Comment {
    pub id: String,
    pub author_id: String,
    pub post_id: String,
    pub parent_id: Option<String>,
    pub contents: String
}

#[derive(Debug, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub id: &'a str,
    pub author_id: &'a str,
    pub post_id: &'a str,
    pub parent_id: Option<&'a str>,
    pub contents: &'a str
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub brief: String,
    pub author_id: String
}

#[derive(Debug, Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub brief: &'a str,
    pub author_id: &'a str
}