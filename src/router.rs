use crate::{services, templates, Pool};
use actix_web::{get, web, HttpResponse, Result};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index_page);
}

#[get("/")]
async fn index_page() -> Result<HttpResponse> {
    let s = templates::render(templates::IndexTemplate);
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
