use crate::domain::projects::ProjectVisibility;
use crate::domain::users::UserID;

#[derive(Debug)]
pub struct NewProject<'a> {
    pub author_id: &'a UserID,
    pub title: &'a str,
    pub brief: &'a str,
    pub visibility: ProjectVisibility,
}
