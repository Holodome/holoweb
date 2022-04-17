use crate::domain::projects::{ProjectID, ProjectVisibility};

pub struct Project {
    id: ProjectID,
    title: String,
    brief: String,

    visibility: ProjectVisibility,
}
