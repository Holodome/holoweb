use crate::utils::see_other;
use actix_web::dev;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web_flash_messages::FlashMessage;

pub fn redirect_on_same_page<B>(
    res: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, res) = res.into_parts();
    FlashMessage::error(res.error().unwrap().to_string()).send();
    let res = see_other(req.path());
    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, res)
            .map_into_boxed_body()
            .map_into_right_body(),
    ))
}
