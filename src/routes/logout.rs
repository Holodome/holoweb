use crate::session::Session;
use crate::utils::{e500, see_other};
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

pub async fn logout(session: Session) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_name().map_err(e500)?.is_none() {
        Ok(see_other("/login"))
    } else {
        session.log_out();
        FlashMessage::info("You have successfully logged out").send();
        Ok(see_other("/login"))
    }
}
