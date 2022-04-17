use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::CommentID;
use crate::domain::users::UserID;
use crate::schema::comments;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct Comment {
    pub id: CommentID,
    pub contents: String,

    pub author_id: UserID,
    pub post_id: BlogPostID,
    pub reply_to_id: Option<CommentID>,

    pub created_at: String,
    pub updated_at: String,

    pub is_deleted: bool,
}
