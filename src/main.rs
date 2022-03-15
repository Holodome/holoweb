#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_web::{web, App, HttpServer, middleware};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod schema;
mod models;
mod handlers;
mod router;
mod services;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL expected");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool");

    log::info!("Starting server on 127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
            )
            .wrap(middleware::Logger::default())
            .configure(handlers::configure)
            .configure(router::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
