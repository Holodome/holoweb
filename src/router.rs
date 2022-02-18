use actix_web::{get, Result, web};
use crate::HttpResponse;
use askama::Template;

pub fn config_router(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}