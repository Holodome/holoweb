use diesel::r2d2::ConnectionManager;
use holosite::config::get_config;
use holosite::startup::{run, Pool};
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: Pool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let config = get_config().expect("Failed to read config");
    let pool = configure_database(config.database_path).await;

    let server = run(listener, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: pool,
    }
}

async fn configure_database(path: String) -> Pool {
    let pool: Pool = Pool::builder()
        .build(ConnectionManager::new(path))
        .expect("Failed to create db pool");
    pool
}

#[actix_web::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
