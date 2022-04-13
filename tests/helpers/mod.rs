mod test_app;
mod test_blog_post;
mod test_comment;
mod test_user;

use once_cell::sync::Lazy;
pub use test_app::*;
pub use test_blog_post::*;
pub use test_comment::*;
pub use test_user::*;

use holosite::config::Settings;
use holosite::startup::{get_connection_pool, Pool};

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

fn get_test_config() -> Settings {
    let mut c = holosite::config::get_config().expect("Failed ot get config");
    c.database_path = ":memory:".to_string();
    c.run_db_migrations = true;
    c.app.port = 0;
    c.app.workers = Some(1);
    c
}

pub fn get_test_db_connection() -> Pool {
    Lazy::force(&TRACING);
    get_connection_pool(":memory:", true)
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
