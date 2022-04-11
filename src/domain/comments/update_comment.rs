use crate::domain::comments::CommentID;
use crate::schema::comments;

#[derive(Debug, diesel::AsChangeset)]
#[table_name = "comments"]
pub struct UpdateComment<'a> {
    pub id: &'a CommentID,
    pub contents: Option<&'a str>,
    pub is_deleted: Option<bool>,
}
