use std::net::TcpListener;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use crate::Pool;
use crate::routes::health_check;

pub fn run(listener: TcpListener, pool: Pool) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .listen(listener)?
        .run();
    Ok(server)
}