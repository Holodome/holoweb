use crate::domain::blog_posts::BlogPostID;
use crate::domain::users::UserID;
use crate::schema::blog_posts;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct BlogPost {
    pub id: BlogPostID,
    pub title: String,
    pub brief: String,
    pub contents: String,
    pub author_id: UserID,
    pub created_at: String,
}
