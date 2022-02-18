use actix_web::{get, Result, web};
use crate::HttpResponse;
use askama::Template;

pub fn config_router(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(projects)
        .service(blog_posts);
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "projects.html")]
struct Projects;

#[derive(Template)]
#[template(path = "blog_posts.html")]
struct BlogPosts;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/projects")]
async fn projects() -> Result<HttpResponse> {
    let s = Projects.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/blog_posts")]
async fn blog_posts() -> Result<HttpResponse> {
    let s = BlogPosts.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}