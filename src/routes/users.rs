use crate::domain::time::DateTime;
use crate::domain::users::UserID;
use crate::middleware::Messages;
use crate::services::{
    get_blog_post_by_id, get_blog_posts_of_author, get_comments_of_author, get_user_by_id,
};
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

struct CommentInfo {
    blog_post_id: String,
    blog_post_title: String,
    date: String,
    contents: String,
}

#[derive(Template)]
#[template(path = "user.html")]
struct UserPageTemplate<'a> {
    name: &'a str,
    projects: Vec<ProjectInfo<'a>>,
    blog_posts: Vec<BlogPostInfo<'a>>,
    comments: Vec<CommentInfo>,
    messages: Messages,
    registered_when: &'a str,
    display_account_link: bool,
}

#[tracing::instrument("User page", skip(pool, messages))]
pub async fn user_page(
    pool: web::Data<Pool>,
    path: web::Path<UserID>,
    messages: IncomingFlashMessages,
    current_user_id: Option<UserID>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    let user = get_user_by_id(&pool, &user_id)
        .map_err(e500)?
        .ok_or_else(|| e500("Failed to get user"))?;

    let blog_posts = get_blog_posts_of_author(&pool, &user_id).map_err(e500)?;
    let blog_post_infos = blog_posts
        .iter()
        .map(|b| BlogPostInfo {
            id: b.id.as_ref().as_str(),
            title: b.title.as_str(),
            brief: b.brief.as_str(),
            role: "TODO",
        })
        .collect();

    let comments = get_comments_of_author(&pool, &user_id).map_err(e500)?;
    let mut comment_infos = Vec::new();
    let now = DateTime::now();
    for comment in comments {
        let blog_post = get_blog_post_by_id(&pool, &comment.post_id)
            .map_err(e500)?
            .ok_or_else(|| e500("Failed to get blog post"))?;
        comment_infos.push(CommentInfo {
            blog_post_id: blog_post.id.as_ref().clone(),
            blog_post_title: blog_post.title.clone(),
            date: comment.created_at.since(&now),
            contents: comment.contents.clone(),
        });
    }

    render_template(UserPageTemplate {
        name: user.name.as_ref(),
        projects: vec![],
        blog_posts: blog_post_infos,
        comments: comment_infos,
        messages: messages.into(),
        registered_when: user.created_at.ago().as_str(),
        display_account_link: current_user_id.map(|it| it == user.id).unwrap_or(false),
    })
}
