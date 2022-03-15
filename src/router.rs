use askama::Template;
use actix_web::{get, Result, HttpResponse, web};
use crate::{services, Pool};
use crate::models;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(posts);
}

#[derive(Template)]
#[template(path="index.html")]
struct Index;

#[derive(Template)]
#[template(path="posts.html")]
struct Posts<'a> {
    posts: &'a Vec<models::Post>
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/posts")]
async fn posts(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let posts = &services::get_all_posts(pool)
        .unwrap_or(Vec::default());

    let s = Posts {
        posts
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}