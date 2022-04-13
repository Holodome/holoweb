use actix_web::http::header::ContentType;
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use askama::Template;

/// Mapper to InternalServerError. This is preferred way to unwrap result returning functions.
/// Preserves error cause for logging.
///
/// # Examples
///
/// ```
/// use actix_web::HttpResponse;
/// use holosite::utils::e500;
/// fn route() -> actix_web::Result<HttpResponse> {
///     let res: Result<HttpResponse, anyhow::Error> = Err(anyhow::anyhow!("Error"));
///     let resp: HttpResponse = res.map_err(e500)?;
///     Ok(resp)
/// }
/// ```
pub fn e500<E>(e: E) -> actix_web::Error
where
    E: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

/// Used to generate error string representation, preserving all child error types.
pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub fn render_template<T: Template>(template: T) -> actix_web::Result<HttpResponse> {
    let s = template.render().map_err(e500)?;
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(s))
}

/// Helper function used to generate redirection response.
pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}
