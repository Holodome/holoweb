use actix_web::{web, HttpResponse};

#[tracing::instrument(name = "Health check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
