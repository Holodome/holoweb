use crate::domain::blog_posts::BlogPostID;
use crate::domain::users::UserID;
use crate::schema::blog_posts;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct BlogPost {
    id: BlogPostID,
    title: String,
    brief: String,
    contents: String,
    author_id: UserID,
    created_at: String
}
