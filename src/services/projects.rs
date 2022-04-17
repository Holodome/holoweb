use crate::domain::blog_posts::BlogPostID;
use crate::domain::projects::{Project, ProjectID, UpdateProject};
use crate::domain::users::{User, UserID};
use crate::Pool;

pub fn get_project_by_id(pool: &Pool, id: &ProjectID) -> Result<Project, anyhow::Error> {
    todo!()
}

pub fn get_project_by_title(pool: &Pool, t: &str) -> Result<Project, anyhow::Error> {
    todo!()
}

pub fn update_project(pool: &Pool, changeset: &UpdateProject) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn get_project_editor_ids(pool: &Pool, id: &ProjectID) -> Result<Vec<UserID>, anyhow::Error> {
    todo!()
}

pub fn add_project_editor(pool: &Pool, id: &ProjectID, user: &UserID) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn remove_project_editor(
    pool: &Pool,
    id: &ProjectID,
    user: &UserID,
) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn get_project_blog_post_ids(
    pool: &Pool,
    id: &ProjectID,
) -> Result<Vec<BlogPostID>, anyhow::Error> {
    todo!()
}

pub fn add_project_blog_post(
    pool: &Pool,
    id: &ProjectID,
    post: &BlogPostID,
) -> Result<(), anyhow::Error> {
    todo!()
}
