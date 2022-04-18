use crate::domain::users::{UserID, UserName};
use crate::services::get_user_by_id;
use crate::utils::{e500, render_template};

use crate::domain::blog_posts::BlogPost;
use crate::domain::projects::Project;
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
#[template(path = "account.html")]
struct AccountPage<'a> {
    messages: IncomingFlashMessages,
    projects: Vec<ProjectInfo<'a>>,
    blog_posts: Vec<BlogPostInfo<'a>>,
    comments: Vec<CommentInfo<'a>>,
    name: &'a str,
    email: &'a str,
}

#[tracing::instrument(skip(pool, messages))]
pub async fn account(
    pool: web::Data<Pool>,
    user_id: UserID,
    messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let user = get_user_by_id(&pool, &user_id)
        .map_err(e500)?
        .ok_or_else(|| e500("Failed to get user name"))?;

    render_template(AccountPage {
        messages,
        projects: vec![],
        blog_posts: vec![],
        comments: vec![],
        name: &user.name.as_ref(),
        email: &user.email.as_ref(),
    })
}
