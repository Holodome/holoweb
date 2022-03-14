use actix_web::{HttpResponse, web, get, post, Error};
use crate::{actions, Pool};
use crate::models::Post;
use serde::{Serialize, Deserialize};

use crate::schema::posts::dsl::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPost {
    pub name: String,
    pub contents: String
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_post)
        .service(add_post)
    ;
}

#[get("/post/{post_id}")]
async fn get_post(
    pool: web::Data<Pool>,
    post_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || actions::find_post_by_id(pool, post_id.into_inner()))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| HttpResponse::NotFound())?
    )
}

#[post("/post/{post_id}")]
async fn add_post(
    pool: web::Data<Pool>,
    new_post: web::Json<InputPost>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || actions::add_new_post(pool, new_post))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}
