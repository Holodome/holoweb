use crate::domain::comments::CommentID;
use crate::schema::comments;

#[derive(Debug, diesel::AsChangeset)]
#[table_name = "comments"]
pub struct UpdateComment<'a> {
    pub id: &'a CommentID,
    pub contents: &'a str,
}
