use crate::domain::blog_posts::BlogPostID;
use crate::schema::blog_posts;

#[derive(diesel::AsChangeset)]
#[table_name = "blog_posts"]
pub struct UpdateBlogPost {
    pub id: BlogPostID,
    pub title: Option<String>,
    pub brief: Option<String>,
    pub contents: Option<String>,
}
