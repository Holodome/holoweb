use serde::{Serialize, Deserialize};
use crate::schema::blog_posts;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct BlogPost {
    pub id: String,
    pub name: String,
    pub contents: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="blog_posts"]
pub struct NewBlogPost {
    pub name: String,
    pub contents: String
}