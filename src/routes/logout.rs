use crate::middleware::{require_login, Session};
use crate::utils::see_other;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use actix_web_lab::middleware::from_fn;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/logout")
            .wrap(from_fn(require_login))
            .route(web::get().to(logout)),
    );
}

pub async fn logout(session: Session) -> HttpResponse {
    session.log_out();
    FlashMessage::info("You have successfully logged out").send();
    see_other("/login")
}
