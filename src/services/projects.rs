use crate::domain::blog_posts::BlogPostID;
use crate::domain::projects::{NewProject, Project, ProjectID, UpdateProject};
use crate::domain::users::UserID;
use crate::schema::projects::dsl::*;
use crate::Pool;
use diesel::{
    insert_into, update, BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl,
    RunQueryDsl,
};

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

pub fn update_project(pool: &Pool, changeset: &UpdateProject) -> Result<(), anyhow::Error> {
    let conn = pool.get()?;
    update(projects.filter(id.eq(&changeset.id)))
        .set(changeset)
        .execute(&conn)?;
    Ok(())
}

pub fn create_project(pool: &Pool, new_project: &NewProject) -> Result<Project, anyhow::Error> {
    let conn = pool.get()?;
    let project = Project {
        id: ProjectID::generate_random(),
        title: new_project.title.to_string(),
        brief: new_project.brief.to_string(),
        author_id: new_project.author_id.clone(),
        visibility: new_project.visibility.clone(),
    };
    insert_into(projects).values(&project).execute(&conn)?;
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
        .select(project_id)
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
