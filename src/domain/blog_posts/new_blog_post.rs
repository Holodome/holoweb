use crate::domain::users::UserID;

#[derive(Debug)]
pub struct NewBlogPost {
    pub title: String,
    pub brief: String,
    pub contents: String,
    pub author_id: UserID,
}
