use crate::domain::blog_posts::{BlogPost, NewBlogPost};
use crate::domain::users::UserID;
use crate::middleware::{require_login, Session};
use crate::services::{get_all_blog_posts, Page};
use crate::startup::Pool;
use crate::utils::{e500, see_other};
use actix_web::http::header::ContentType;
use actix_web::{route, web, HttpResponse};
use actix_web_lab::middleware::from_fn;

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

#[route("/blog_posts", method = "GET")]
#[tracing::instrument("All blog posts", skip(pool, query, session))]
pub async fn blog_posts(
    pool: web::Data<Pool>,
    query: web::Query<QueryParams>,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let user_id = session.get_user_id().unwrap();

    let page = query.0.page.unwrap_or_default();
    let blog_posts =
        get_all_blog_posts(&pool, &page).map_err(e500)?;

    let s = BlogPostsTemplate {
        current_user_id: user_id,
        page,
        blog_posts,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

#[derive(Template)]
#[template(path = "create_blog_post.html")]
struct CreateBlogPostTemplate {
    current_user_id: Option<UserID>,
}

#[route("/create_blog_post", method = "GET", wrap = "from_fn(require_login)")]
#[tracing::instrument("Create new blog post form", skip(session))]
pub async fn create_blog_post_form(session: Session) -> actix_web::Result<HttpResponse> {
    let user_id = session.get_user_id().unwrap().unwrap();
    let s = CreateBlogPostTemplate {
        current_user_id: Some(user_id),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

#[derive(serde::Deserialize)]
pub struct CreateBlogPostFormData {
    title: String,
    brief: String,
    contents: String,
}

#[route("/create_blog_post", method = "POST", wrap = "from_fn(require_login)")]
#[tracing::instrument("Create new blog post", skip(pool, form))]
pub async fn create_blog_post(
    form: web::Form<CreateBlogPostFormData>,
    pool: web::Data<Pool>,
    user_id: web::ReqData<UserID>,
) -> actix_web::Result<HttpResponse> {
    let new_blog_post = NewBlogPost {
        title: form.0.title,
        brief: form.0.brief,
        contents: form.0.contents,
        author_id: user_id.into_inner(),
    };
    crate::services::insert_new_blog_post(&pool, &new_blog_post)
        .map_err(e500)?;
    Ok(see_other("/blog_posts"))
}
