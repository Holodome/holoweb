// use crate::domain::projects::Project;
// use actix_web_flash_messages::IncomingFlashMessages;
// use askama::Template;

// #[derive(Template)]
// #[template(path = "projects.html")]
// struct ProjectsTemplate {
//     messages: IncomingFlashMessages,
//     projects: Vec<Project>,
// }

// #[tracing::instrument("All projects", skip(pool, messages))]
// pub async fn all_projects(
//     pool: web::Data<Pool>,
//     messages: IncomingFlashMessages
// ) -> actix_web::Result<HttpResponse> {
//     let projects = get_all_projects(&pool).map_err(e500);
//     render_template(ProjectsTemplate{
//
//     })
// }
