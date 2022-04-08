use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check));
}

#[tracing::instrument(name = "Health check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
