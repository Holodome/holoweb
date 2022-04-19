use crate::domain::projects::{ProjectID, ProjectVisibility};
use crate::schema::projects;

#[derive(Debug, diesel::AsChangeset)]
#[table_name = "projects"]
pub struct UpdateProject<'a> {
    pub id: &'a ProjectID,
    pub title: Option<&'a str>,
    pub brief: Option<&'a str>,
    pub visibility: Option<ProjectVisibility>,
}
