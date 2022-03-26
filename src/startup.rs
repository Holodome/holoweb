use crate::config::Settings;
use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Application {
    port: u16,
    server: Server
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let pool: Pool = Pool::builder()
            .build(ConnectionManager::new(config.database_path))
            .expect("Failed to create db pool");

        let address = format!("{}:{}", config.app.host, config.app.port);
        tracing::info!("Starting server on {:?}", &address);
        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr().unwrap().port();
        let server = run(listener, pool)?;

        Ok( Self { port, server } )
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(listener: TcpListener, pool: Pool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
