use diesel::r2d2::ConnectionManager;
use holosite::config::get_config;
use holosite::startup::{Application, Pool};
use once_cell::sync::Lazy;
use uuid::Uuid;

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

pub struct TestApp {
    pub address: String,
    pub pool: Pool,
    pub api_client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let config = {
        let mut c = get_config().expect("Failed ot get config");
        c.database_path = format!("{}{}", c.database_path, Uuid::new_v4().to_string());
        c.app.port = 0;
        c
    };

    let app = Application::build(config.clone())
        .await
        .expect("Failed to build application");
    let address = format!("http://localhost:{}", app.port());
    let _ = tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();
    TestApp {
        address,
        pool: get_connection_pool(&config.database_path),
        api_client: client,
    }
}

fn get_connection_pool(path: &str) -> Pool {
    let pool: Pool = Pool::builder()
        .build(ConnectionManager::new(path))
        .expect("Failed to create db pool");
    pool
}

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}
