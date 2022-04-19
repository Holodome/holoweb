use crate::domain::blog_posts::BlogPostID;
use crate::domain::projects::{NewProject, Project, ProjectID, UpdateProject};
use crate::domain::users::UserID;
use crate::schema::projects::dsl::*;
use crate::Pool;
use diesel::result::DatabaseErrorKind;
use diesel::{
    insert_into, update, BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl,
    RunQueryDsl,
};
use std::fmt::Formatter;

#[derive(thiserror::Error)]
pub enum ProjectError {
    #[error("Title is already taken")]
    TakenTitle,
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ProjectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

pub fn get_project_by_id(
    pool: &Pool,
    project_id_: &ProjectID,
) -> Result<Option<Project>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(projects
        .filter(id.eq(project_id_))
        .first::<Project>(&conn)
        .optional()?)
}

pub fn get_project_by_title(pool: &Pool, t: &str) -> Result<Option<Project>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(projects
        .filter(title.eq(t))
        .first::<Project>(&conn)
        .optional()?)
}

pub fn get_all_projects(pool: &Pool) -> Result<Vec<Project>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(projects.load::<Project>(&conn)?)
}

pub fn update_project(pool: &Pool, changeset: &UpdateProject) -> Result<(), ProjectError> {
    let conn = pool
        .get()
        .map_err(|e| ProjectError::UnexpectedError(e.into()))?;
    update(projects.filter(id.eq(&changeset.id)))
        .set(changeset)
        .execute(&conn)
        .map_err(get_project_error_from_database_error)?;
    Ok(())
}

pub fn insert_new_project(pool: &Pool, new_project: &NewProject) -> Result<Project, ProjectError> {
    let project = {
        let conn = pool
            .get()
            .map_err(|e| ProjectError::UnexpectedError(e.into()))?;
        let project = Project {
            id: ProjectID::generate_random(),
            title: new_project.title.to_string(),
            brief: new_project.brief.to_string(),
            author_id: new_project.author_id.clone(),
            visibility: new_project.visibility.clone(),
        };
        insert_into(projects)
            .values(&project)
            .execute(&conn)
            .map_err(get_project_error_from_database_error)?;
        project
    };
    add_project_editor(pool, &project.id, &project.author_id)?;
    Ok(project)
}

pub fn get_project_editor_ids(
    pool: &Pool,
    project_id_: &ProjectID,
) -> Result<Vec<UserID>, anyhow::Error> {
    use crate::schema::project_editor_junctions::dsl::*;
    let conn = pool.get()?;
    Ok(project_editor_junctions
        .filter(project_id.eq(project_id_))
        .select(user_id)
        .load::<UserID>(&conn)?)
}

pub fn add_project_editor(
    pool: &Pool,
    project_id_: &ProjectID,
    user: &UserID,
) -> Result<(), anyhow::Error> {
    use crate::schema::project_editor_junctions::dsl::*;
    let conn = pool.get()?;
    insert_into(project_editor_junctions)
        .values((project_id.eq(project_id_), user_id.eq(user)))
        .execute(&conn)?;
    Ok(())
}

pub fn remove_project_editor(
    pool: &Pool,
    project_id_: &ProjectID,
    user: &UserID,
) -> Result<(), anyhow::Error> {
    use crate::schema::project_editor_junctions::dsl::*;
    let conn = pool.get()?;
    diesel::delete(
        project_editor_junctions.filter(project_id.eq(project_id_).and(user_id.eq(user))),
    )
    .execute(&conn)?;
    Ok(())
}

pub fn get_project_blog_post_ids(
    pool: &Pool,
    project_id_: &ProjectID,
) -> Result<Vec<BlogPostID>, anyhow::Error> {
    use crate::schema::project_blog_post_junctions::dsl::*;
    let conn = pool.get()?;
    Ok(project_blog_post_junctions
        .filter(project_id.eq(project_id_))
        .select(post_id)
        .load::<BlogPostID>(&conn)?)
}

pub fn add_project_blog_post(
    pool: &Pool,
    project_id_: &ProjectID,
    post: &BlogPostID,
) -> Result<(), anyhow::Error> {
    use crate::schema::project_blog_post_junctions::dsl::*;
    let conn = pool.get()?;
    insert_into(project_blog_post_junctions)
        .values((project_id.eq(project_id_), post_id.eq(post)))
        .execute(&conn)?;
    Ok(())
}

fn get_project_error_from_database_error(e: diesel::result::Error) -> ProjectError {
    match e {
        diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, ref data) => {
            let msg = data.message();
            if msg.contains("title") {
                ProjectError::TakenTitle
            } else {
                ProjectError::UnexpectedError(e.into())
            }
        }
        _ => ProjectError::UnexpectedError(e.into()),
    }
}
