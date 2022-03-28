use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub async fn home() -> HttpResponse {
    let s = HomeTemplate.render().unwrap();
    HttpResponse::Ok().content_type(ContentType::html()).body(s)
}
