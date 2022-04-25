use crate::config::Config;
use crate::routes::error_handlers::{internal_error_handler, not_found_handler};
use crate::Pool;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::dev::Server;
use actix_web::middleware::ErrorHandlers;
use actix_web::{http, web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use diesel::r2d2::ConnectionManager;
use secrecy::{ExposeSecret, Secret};
use std::net::TcpListener;
use std::time::Duration;
use tracing_actix_web::TracingLogger;

pub struct Application {
    pub port: u16,
    pub server: Server,
}

pub fn get_connection_pool(uri: &str) -> Pool {
    let pool: Pool = Pool::builder()
        .connection_timeout(Duration::new(10, 0))
        .build(ConnectionManager::new(uri))
        .expect("Failed to create db pool");
    pool
}

impl Application {
    pub async fn build_with_pool(config: Config, pool: Pool) -> Result<Self, anyhow::Error> {
        let address = format!("{}:{}", config.app.host, config.app.port);
        tracing::info!("Starting server on {:?}", &address);
        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            pool.clone(),
            config.app.hmac_secret,
            config.redis_uri,
            config.app.workers,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        let pool = get_connection_pool(&config.database_uri);
        Self::build_with_pool(config, pool).await
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener: TcpListener,
    pool: Pool,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
    workers: Option<usize>,
) -> Result<Server, anyhow::Error> {
    let workers = workers.unwrap_or_else(num_cpus::get_physical);
    tracing::info!("Workers: {:?}", &workers);
    let secret_key = actix_web::cookie::Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_uri = format!("redis://{}", redis_uri.expose_secret());
    let redis_store = RedisSessionStore::new(redis_uri)
        .await
        .expect("Failed to connect to redis");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::builder(redis_store.clone(), secret_key.clone()).build())
            .wrap(
                ErrorHandlers::new()
                    .handler(
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        internal_error_handler,
                    )
                    .handler(http::StatusCode::NOT_FOUND, not_found_handler),
            )
            .app_data(web::Data::new(pool.clone()))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .configure(crate::routes::configure)
    })
    .workers(workers)
    .listen(listener)
    .expect("Failed to bind TCP listener")
    .run();
    Ok(server)
}
