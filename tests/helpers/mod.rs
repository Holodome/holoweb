mod test_app;
mod test_blog_post;
mod test_comment;
mod test_user;

use diesel::r2d2::{ConnectionManager, ManageConnection};
use diesel::{Connection, PgConnection};
use once_cell::sync::Lazy;
pub use test_app::*;
pub use test_blog_post::*;
pub use test_comment::*;
pub use test_user::*;
use uuid::Uuid;

use holosite::config::{Config, DbConfig};
use holosite::startup::get_connection_pool;
use holosite::Pool;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = holosite::telemetry::get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::stdout,
        );
        holosite::telemetry::init_subscriber(subscriber);
    } else {
        let subscriber = holosite::telemetry::get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::sink,
        );
        holosite::telemetry::init_subscriber(subscriber);
    }
});

fn get_test_config() -> Config {
    let mut c = holosite::config::get_config().expect("Failed ot get config");
    c.database.database_name = Uuid::new_v4().to_string();
    c.app.port = 0;
    c.app.workers = Some(1);

    c
}

embed_migrations!();

pub struct TestDB {
    pub pool: Pool,
    pub config: DbConfig,
}

impl TestDB {
    pub fn new(config: &DbConfig) -> Self {
        let conn = ConnectionManager::<PgConnection>::new(config.uri_without_db())
            .connect()
            .expect("Failed to connect to database");
        conn.execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
            .expect("Failed to create database");

        let pool = get_connection_pool(&config.uri());
        let conn = pool.get().expect("Failed to get connection");
        embedded_migrations::run(&conn).expect("Failed to run migrations");
        Self {
            pool,
            config: config.clone(),
        }
    }

    pub fn spawn() -> TestDB {
        Lazy::force(&TRACING);
        let config = get_test_config();
        Self::new(&config.database)
    }
}

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(
        response.status().as_u16(),
        303,
        "Response is not redirect as expected: {:?}",
        response
    );
    assert_eq!(
        response.headers().get("Location").unwrap(),
        location,
        "Response is redirect to different location"
    );
}

pub fn assert_resp_ok(response: &reqwest::Response) {
    assert_eq!(
        response.status().as_u16(),
        200,
        "Response is not OK: {:?}",
        response
    )
}

pub fn assert_resp_forbidden(response: &reqwest::Response) {
    assert_eq!(
        response.status().as_u16(),
        403,
        "Response is not FORBIDDEN: {:?}",
        response
    )
}
