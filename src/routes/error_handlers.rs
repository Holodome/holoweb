use crate::middleware::Messages;
use crate::utils::render_template;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::ErrorHandlerResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "error_page.html")]
pub struct ErrorPageTemplate<'a> {
    pub error_title: &'a str,
    pub error_message: &'a str,
    pub messages: Messages,
}

pub fn not_found_handler<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
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
    res: ServiceResponse<B>,
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
