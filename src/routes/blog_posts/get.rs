use crate::domain::blog_posts::BlogPost;
use crate::domain::users::UserID;
use crate::services::{get_all_blog_posts, Page};
use crate::startup::Pool;
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "blog_posts.html")]
struct BlogPostsTemplate {
    current_user_id: Option<UserID>,
    page: Page,
    blog_posts: Vec<BlogPost>,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub page: Option<Page>,
}

#[tracing::instrument("All blog posts", skip(pool, query))]
pub async fn blog_posts(
    pool: web::Data<Pool>,
    query: web::Query<QueryParams>,
    user_id: Option<UserID>,
) -> actix_web::Result<HttpResponse> {
    let page = query.0.page.unwrap_or_default();
    let blog_posts = get_all_blog_posts(&pool, &page).map_err(e500)?;

    let s = BlogPostsTemplate {
        current_user_id: user_id,
        page,
        blog_posts,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}
