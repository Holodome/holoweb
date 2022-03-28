use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    errors: Vec<String>
}

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let errors = flash_messages.iter()
        .map(|m| m.content().to_string()).collect::<Vec<_>>();
    let s = LoginTemplate {
        errors
    }.render().unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(s)
}
