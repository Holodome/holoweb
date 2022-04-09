use crate::domain::comments::CommentID;
use crate::schema::comments;

#[derive(Debug, diesel::AsChangeset)]
#[table_name = "comments"]
pub struct UpdateComment<'a> {
    id: &'a CommentID,
    contents: &'a str,
}
