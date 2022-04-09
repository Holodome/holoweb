use crate::middleware::Session;
use crate::utils::see_other;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

pub async fn logout(session: Session) -> HttpResponse {
    session.log_out();
    FlashMessage::info("You have successfully logged out").send();
    see_other("/login")
}
