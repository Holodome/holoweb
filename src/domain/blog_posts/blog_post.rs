use crate::domain::blog_posts::{BlogPostID, BlogPostVisibility};
use crate::domain::time::DateTime;
use crate::domain::users::UserID;
use crate::schema::blog_posts;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct BlogPost {
    pub id: BlogPostID,
    pub title: String,
    pub brief: String,
    pub contents: String,

    pub author_id: UserID,

    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub visibility: BlogPostVisibility,
}
