use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::CommentID;
use crate::domain::users::UserID;
use crate::schema::comments;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct Comment {
    id: CommentID,
    author_id: UserID,
    post_id: BlogPostID,
    parent_id: Option<CommentID>,
    contents: String,
    created_at: String,
}
