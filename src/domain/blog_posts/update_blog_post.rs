use crate::domain::blog_posts::BlogPostID;
use crate::schema::blog_posts;

#[derive(diesel::AsChangeset)]
#[table_name = "blog_posts"]
pub struct UpdateBlogPost<'a> {
    pub id: &'a BlogPostID,
    pub title: Option<&'a str>,
    pub brief: Option<&'a str>,
    pub contents: Option<&'a str>,
}
