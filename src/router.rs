use actix_web::{get, Result, HttpResponse, web};
use crate::{services, Pool, templates};


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index_page)
    ;
}

#[get("/")]
async fn index_page() -> Result<HttpResponse> {
    let s = templates::render(templates::IndexTemplate);
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
