use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::CommentID;
use crate::domain::users::UserID;

#[derive(Debug)]
pub struct NewComment<'a> {
    id: CommentID,
    author_id: &'a UserID,
    post_id: &'a BlogPostID,
    parent_id: Option<&'a CommentID>,
    contents: &'a str,
}
