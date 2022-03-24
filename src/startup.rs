use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use std::net::TcpListener;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn run(listener: TcpListener, pool: Pool) -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new(move || App::new().route("/health_check", web::get().to(health_check))
            .app_data(web::Data::new(pool.clone())))
            .listen(listener)?
            .run();
    Ok(server)
}
