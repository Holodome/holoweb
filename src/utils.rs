use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use actix_web_flash_messages::{IncomingFlashMessages, Level};

pub fn e500<E>(e: E) -> actix_web::Error
where
    E: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

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

pub fn extract_flash_messages_level(
    flash_messages: &IncomingFlashMessages,
    level: Level,
) -> Vec<String> {
    flash_messages
        .iter()
        .filter(|m| m.level() == level)
        .map(|m| m.content().to_string())
        .collect::<Vec<_>>()
}

pub fn extract_errors(flash_messages: &IncomingFlashMessages) -> Vec<String> {
    extract_flash_messages_level(flash_messages, Level::Error)
}

pub fn extract_infos(flash_messages: &IncomingFlashMessages) -> Vec<String> {
    extract_flash_messages_level(flash_messages, Level::Info)
}
