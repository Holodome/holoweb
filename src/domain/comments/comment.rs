use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::CommentID;
use crate::domain::users::UserID;
use crate::schema::comments;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct Comment {
    pub id: CommentID,
    pub author_id: UserID,
    pub post_id: BlogPostID,
    pub parent_id: Option<CommentID>,
    pub contents: String,
    pub created_at: String,
    pub is_deleted: bool,
    pub main_parent_id: Option<BlogPostID>,
    pub depth: i32,
}
