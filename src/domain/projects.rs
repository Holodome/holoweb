use crate::schema::projects;

#[derive(Debug, serde::Serialize, serde::Deserialize, Queryable)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub brief: String,
    pub author_id: String,
}

#[derive(Debug, Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub brief: &'a str,
    pub author_id: &'a str,
}
