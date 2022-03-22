use crate::schema::comments;

#[derive(Debug, serde::Serialize, serde::Deserialize, Queryable)]
pub struct Comment {
    pub id: String,
    pub author_id: String,
    pub post_id: String,
    pub parent_id: Option<String>,
    pub contents: String
}

#[derive(Debug, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub author_id: &'a str,
    pub post_id: &'a str,
    pub parent_id: Option<&'a str>,
    pub contents: &'a str
}
