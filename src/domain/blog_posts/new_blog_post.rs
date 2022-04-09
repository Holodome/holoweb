use crate::domain::users::UserID;

#[derive(Debug)]
pub struct NewBlogPost<'a> {
    pub title: &'a str,
    pub brief: &'a str,
    pub contents: &'a str,
    pub author_id: &'a UserID,
}
