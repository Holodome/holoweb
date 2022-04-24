use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::CommentID;
use crate::domain::time::DateTime;
use crate::domain::users::UserName;

#[derive(Debug, diesel::Queryable)]
pub struct CommentView {
    pub id: CommentID,
    pub contents: String,

    pub author_name: UserName,
    pub post_id: BlogPostID,
    pub reply_to_id: Option<CommentID>,

    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub is_deleted: bool,
}
