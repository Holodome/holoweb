#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Error, middleware};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod schema;
mod models;
mod handlers;
mod router;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL expected");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool");

    log::info!("Starting server on 127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true)
            )
            .service(get_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
