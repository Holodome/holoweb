use crate::domain::blog_posts::BlogPost;
use crate::domain::users::UserName;
use crate::middleware::reject_anonymous_users;
use crate::services::{get_all_blog_posts, Page};
use crate::startup::Pool;
use actix_web::http::header::ContentType;
use actix_web::{route, web, HttpResponse};
use actix_web_lab::middleware::from_fn;
use actix_web_lab::web::redirect;
use askama::Template;

#[derive(Template)]
#[template(path = "blog_posts.html")]
struct BlogPostsTemplate<'a> {
    current_user_name: Option<&'a UserName>,
    page: Page,
    blog_posts: Vec<BlogPost>,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub page: Option<Page>,
}

#[route("/blog_posts", method = "GET")]
#[tracing::instrument("All blog posts", skip(pool, query))]
pub async fn blog_posts(
    pool: web::Data<Pool>,
    query: web::Query<QueryParams>,
    current_user_name: Option<web::ReqData<UserName>>,
) -> actix_web::Result<HttpResponse> {
    let page = query.0.page.unwrap_or_default();
    let blog_posts =
        get_all_blog_posts(&pool, &page).map_err(actix_web::error::ErrorInternalServerError)?;

    let s = BlogPostsTemplate {
        current_user_name: current_user_name.as_deref(),
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
    current_user_name: Option<UserName>,
}

#[route(
    "/create_blog_post",
    method = "GET",
    wrap = "from_fn(reject_anonymous_users)"
)]
#[tracing::instrument("Create new blog post form")]
pub async fn create_blog_post_form(
    current_user_name: web::ReqData<UserName>,
) -> actix_web::Result<HttpResponse> {
    let s = CreateBlogPostTemplate {
        current_user_name: Some(current_user_name.into_inner()),
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

#[derive(serde::Deserialize)]
pub struct CreateBlogPostFormData {
    title: String,
    brief: String,
    contents: String
}

#[route("/create_blog_post", method="GET", wrap="from_fn(reject_anonymous_users)")]
#[tracing::instrument("Create new blog post", skip(pool, form))]
pub async fn create_blog_post(
    form: web::Form<CreateBlogPostFormData>,
    pool: web::Data<Pool>,
    user_name: web::ReqData<UserName>,
) -> actix_web::Result<HttpResponse> {
    let user_name = user_name.into_inner();

    Ok()
}