use crate::config::Settings;
use crate::routes::{
    account, blog_posts, change_name, change_name_form, change_password, change_password_form,
    create_blog_post, create_blog_post_form, health_check, home, login, login_form, logout,
    registration, registration_form,
};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use secrecy::{ExposeSecret, Secret};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let pool: Pool = Pool::builder()
            .build(ConnectionManager::new(config.database_path))
            .expect("Failed to create db pool");

        let address = format!("{}:{}", config.app.host, config.app.port);
        tracing::info!("Starting server on {:?}", &address);
        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr().unwrap().port();
        let server = run(listener, pool, config.app.hmac_secret, config.redis_uri).await?;

        Ok(Self { port, server })
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
) -> Result<Server, anyhow::Error> {
    let secret_key = actix_web::cookie::Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(hmac_secret.clone()))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(home))
            .service(login)
            .service(login_form)
            .service(logout)
            .service(registration)
            .service(registration_form)
            .service(account)
            .service(change_password)
            .service(change_password_form)
            .service(change_name)
            .service(change_name_form)
            .service(blog_posts)
            .service(create_blog_post_form)
            .service(create_blog_post)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
