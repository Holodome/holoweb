use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::CommentID;
use crate::domain::users::UserID;

#[derive(Debug)]
pub struct NewComment<'a> {
    pub author_id: &'a UserID,
    pub post_id: &'a BlogPostID,
    pub parent_id: Option<&'a CommentID>,
    pub contents: &'a str,
}
