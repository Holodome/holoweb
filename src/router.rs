use actix_web::{get, Result};
use crate::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}