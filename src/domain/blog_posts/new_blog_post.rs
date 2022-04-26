use crate::domain::blog_posts::BlogPostVisibility;
use crate::domain::users::UserID;

#[derive(Debug)]
pub struct NewBlogPost<'a> {
    pub author_id: &'a UserID,
    pub title: &'a str,
    pub brief: &'a str,
    pub contents: &'a str,
    pub visibility: BlogPostVisibility,
}
