use crate::domain::projects::{ProjectID, ProjectVisibility};
use crate::domain::users::UserID;
use crate::schema::projects;

#[derive(Debug, diesel::Queryable, diesel::Insertable, PartialEq)]
pub struct Project {
    pub id: ProjectID,
    pub title: String,
    pub brief: String,
    pub author_id: UserID,
    pub visibility: ProjectVisibility,
}
