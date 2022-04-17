use crate::domain::blog_posts::{BlogPost, BlogPostID};
use crate::domain::users::UserID;
use crate::services::{get_all_blog_posts, get_blog_post_by_id, get_comments_for_blog_post, Page};
use crate::utils::{e500, render_template};

use crate::domain::comments::Comment;
use crate::Pool;
use actix_web::{web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "blog_posts.html")]
struct BlogPostsTemplate {
    current_user_id: Option<UserID>,
    blog_posts: Vec<BlogPost>,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub page: Option<Page>,
}

#[tracing::instrument("All blog posts", skip(pool, query))]
pub async fn all_blog_posts(
    pool: web::Data<Pool>,
    query: web::Query<QueryParams>,
    user_id: Option<UserID>,
) -> actix_web::Result<HttpResponse> {
    let page = query.0.page.unwrap_or_default();
    let blog_posts = get_all_blog_posts(&pool, &page).map_err(e500)?;

    render_template(BlogPostsTemplate {
        current_user_id: user_id,
        blog_posts,
    })
}

#[derive(Template)]
#[template(path = "blog_post.html")]
struct BlogPostTemplate {
    current_user_id: Option<UserID>,
    blog_post: BlogPost,
    rendered_comments: String,
}

#[tracing::instrument("Blog post", skip(pool))]
pub async fn blog_post(
    pool: web::Data<Pool>,
    params: web::Path<BlogPostID>,
    user_id: Option<UserID>,
) -> actix_web::Result<HttpResponse> {
    let blog_post_id = params.into_inner();
    let blog_post = get_blog_post_by_id(&pool, &blog_post_id)
        .map_err(e500)?
        .ok_or_else(|| actix_web::error::ErrorNotFound("No blog post with such id"))?;

    let comments =
        get_comments_for_blog_post(&pool, &blog_post_id, &Page::infinite()).map_err(e500)?;

    render_template(BlogPostTemplate {
        current_user_id: user_id,
        blog_post,
        rendered_comments: "".to_string(),
    })
}
