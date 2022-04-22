use crate::middleware::Messages;
use crate::utils::render_template;
use actix_web::dev;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::ErrorHandlerResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "error_page.html")]
struct ErrorPageTemplate<'a> {
    error_title: &'a str,
    error_message: &'a str,
    messages: Messages,
}

pub fn not_found_handler<B>(
    res: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, _) = res.into_parts();
    let res = render_template(ErrorPageTemplate {
        error_title: "Not found",
        error_message: "Page with requested URL is nonexistent",
        messages: Messages::empty(),
    })?;
    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, res)
            .map_into_boxed_body()
            .map_into_right_body(),
    ))
}

pub fn internal_error_handler<B>(
    res: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, _) = res.into_parts();
    let res = render_template(ErrorPageTemplate {
        error_title: "Internal server error",
        error_message: "Something horrible happened to request",
        messages: Messages::empty(),
    })?;
    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, res)
            .map_into_boxed_body()
            .map_into_right_body(),
    ))
}
