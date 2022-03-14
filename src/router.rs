use actix_web::{web, HttpResponse, Error};
use actix_web::{get, post};
use crate::handlers;
use crate::Pool;

pub fn configure()

#[get("/post/{post_id}")]
async fn get_post(
    pool: web::Data<Pool>,
    id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || handlers::find_post_by_id(pool, id.into_inner()))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}