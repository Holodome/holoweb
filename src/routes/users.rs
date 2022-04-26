use crate::domain::blog_posts::BlogPost;
use crate::domain::projects::Project;
use crate::domain::users::UserID;
use crate::middleware::Messages;
use crate::services::get_user_by_id;
use crate::utils::{e500, render_template};
use crate::Pool;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

struct ProjectInfo<'a> {
    id: &'a str,
    title: &'a str,
    brief: &'a str,
    role: &'a str,
}

struct BlogPostInfo<'a> {
    id: &'a str,
    title: &'a str,
    brief: &'a str,
    role: &'a str,
}

struct CommentInfo<'a> {
    project: &'a Project,
    blog_post: &'a BlogPost,
    date: &'a str,
    contents: &'a str,
}

#[derive(Template)]
#[template(path = "user.html")]
struct UserPageTemplate<'a> {
    name: &'a str,
    projects: Vec<ProjectInfo<'a>>,
    blog_posts: Vec<BlogPostInfo<'a>>,
    comments: Vec<CommentInfo<'a>>,
    messages: Messages,
}

#[tracing::instrument("User page", skip(pool, messages))]
pub async fn user_page(
    pool: web::Data<Pool>,
    path: web::Path<UserID>,
    messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    let user = get_user_by_id(&pool, &user_id)
        .map_err(e500)?
        .ok_or_else(|| e500("Failed to get user"))?;

    render_template(UserPageTemplate {
        name: user.name.as_ref(),
        projects: vec![],
        blog_posts: vec![],
        comments: vec![],
        messages: messages.into(),
    })
}
