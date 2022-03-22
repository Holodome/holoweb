use crate::schema::blog_posts;

#[derive(Debug, serde::Serialize, serde::Deserialize, Queryable, Insertable)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub brief: String,
    pub contents: String,
    pub author_id: String
}

#[derive(Debug)]
pub struct NewBlogPost<'a> {
    pub title: &'a str,
    pub brief: Option<&'a str>,
    pub contents: &'a str,
    pub author_id: &'a str
}

#[derive(Debug, AsChangeset)]
#[table_name = "blog_posts"]
pub struct UpdateBlogPost<'a> {
    pub id: &'a str,
    pub brief: Option<&'a str>,
    pub contents: Option<&'a str>,
    pub author_id: Option<&'a str>
}