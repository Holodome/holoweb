use crate::domain::blog_posts::BlogPostID;
use crate::schema::blog_posts;

#[derive(diesel::AsChangeset)]
#[table_name = "blog_posts"]
pub struct UpdateBlogPost {
    id: BlogPostID,
    title: Option<String>,
    brief: Option<String>,
    contents: Option<String>,
}
