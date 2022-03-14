use actix_web::{HttpResponse, web, get, post, Error};
use crate::{actions, Pool};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPost {
    pub name: String,
    pub contents: String
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_post)
        .service(add_post)
        .service(get_all_posts)
    ;
}

#[get("/post/{post_id}")]
async fn get_post(
    pool: web::Data<Pool>,
    post_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || actions::find_post_by_id(pool, post_id.into_inner()))
            .await?
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(actix_web::error::ErrorInternalServerError)?
    )
}

#[get("/post")]
async fn get_all_posts(
    pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || actions::get_all_posts(pool))
            .await?
            .map(|users| HttpResponse::Ok().json(users))
            .map_err(actix_web::error::ErrorInternalServerError)?
    )
}

#[post("/post")]
async fn add_post(
    pool: web::Data<Pool>,
    new_post: web::Json<InputPost>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || actions::add_new_post(pool, new_post))
            .await?
            .map(|post| HttpResponse::Ok())
            .map_err(actix_web::error::ErrorInternalServerError)?
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, HttpServer, App};
    use diesel::r2d2::ConnectionManager;
    use diesel::{r2d2, SqliteConnection};
    use crate::models;

    #[actix_web::test]
    fn post_routes() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL expected");

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create db pool");

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(configure)
        ).await;

        let req = test::TestRequest::post()
            .uri("/post")
            .set_json(&models::NewPost{
                name: "Test name",
                contents: "Test contents"
            })
            .to_request();
        let resp = test::call_and_read_body_json(&mut app, req);
    }
}