use askama::Template;
use actix_web::{get, Result, HttpResponse, web};
use crate::{services, Pool};
use crate::models;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(posts)
        .service(post);
}

#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path="posts.html")]
struct PostsTemplate {
    posts: Vec<models::Post>
}

#[derive(Template)]
#[template(path="post.html")]
struct PostTemplate {
    post: models::Post
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let s = IndexTemplate.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/posts")]
async fn posts(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let s = PostsTemplate {
        posts: services::get_all_posts(pool)
            .unwrap_or(Vec::default())
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/post/{name}")]
async fn post(pool: web::Data<Pool>, name: web::Path<String>) -> Result<HttpResponse> {
    let post = services::get_post_by_name(pool, name.into_inner());
    let resp = match post {
            Ok(Some(post)) => {
                let s = PostTemplate {
                    post
                }.render().unwrap();
                HttpResponse::Ok().content_type("text/html").body(s)
            },
            _ => HttpResponse::NotFound().finish()
    };
    Ok(resp)
}