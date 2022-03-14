use askama::Template;
use actix_web::{get, Result, HttpResponse, web};
use actix_web::web::post;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(posts);
}

#[derive(Template)]
#[template(path="index.html")]
struct Index;

#[derive(Template)]
#[template(path="post.html")]
struct Post;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/posts")]
async fn posts() -> Result<HttpResponse> {
    let s = Post.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}