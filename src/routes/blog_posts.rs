use crate::domain::blog_posts::{BlogPost, NewBlogPost};
use crate::domain::users::{UserID, UserName};
use crate::middleware::reject_anonymous_users;
use crate::services::{get_all_blog_posts, get_user_by_id, get_user_by_name, Page};
use crate::startup::Pool;
use crate::utils::see_other;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::ContentType;
use actix_web::{route, web, HttpResponse};
use actix_web_lab::middleware::from_fn;
use actix_web_lab::web::redirect;
use askama::Template;

#[derive(Template)]
#[template(path = "blog_posts.html")]
struct BlogPostsTemplate<'a> {
    current_user_id: Option<&'a UserID>,
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
    current_user_id: Option<web::ReqData<UserID>>,
) -> actix_web::Result<HttpResponse> {
    let page = query.0.page.unwrap_or_default();
    let blog_posts =
        get_all_blog_posts(&pool, &page).map_err(actix_web::error::ErrorInternalServerError)?;

    let s = BlogPostsTemplate {
        current_user_id: current_user_id.as_deref(),
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

#[route(
    "/create_blog_post",
    method = "GET",
    wrap = "from_fn(reject_anonymous_users)"
)]
#[tracing::instrument("Create new blog post form")]
pub async fn create_blog_post_form(
    current_user_id: web::ReqData<UserID>,
) -> actix_web::Result<HttpResponse> {
    let s = CreateBlogPostTemplate {
        current_user_id: Some(current_user_id.into_inner()),
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

#[route(
    "/create_blog_post",
    method = "POST",
    wrap = "from_fn(reject_anonymous_users)"
)]
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
    crate::services::insert_new_blog_post(&pool, &new_blog_post).map_err(ErrorInternalServerError);
    Ok(see_other("/blog_posts"))
}
