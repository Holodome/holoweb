use crate::api::assert_is_redirect_to_resource;
use crate::common::{TestApp, TestUser};
use secrecy::ExposeSecret;
use uuid::Uuid;

#[tokio::test]
async fn logout_returns_redirect_to_login_when_not_logged_in() {
    let app = TestApp::spawn().await;
    let response = app.post_logout().await;
    assert_is_redirect_to_resource(&response, "/login")
}

#[tokio::test]
async fn error_flash_message_is_set_on_failure() {
    let app = TestApp::spawn().await;

    let login_body = serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass"
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to_resource(&response, "/login");

    // Check that error message is present now
    let html_page = app.get_login_page_html().await;
    assert!(html_page.contains("Authentication failed"));

    // Check that error message is not present now
    let html_page = app.get_login_page_html().await;
    assert!(!html_page.contains("Authentication failed"));
}

#[tokio::test]
async fn error_message_invalid_name_is_set_on_login() {
    let app = TestApp::spawn().await;

    let login_body = serde_json::json!({
        "name": "",
        "password": "1"
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to_resource(&response, "/login");

    // Check that error message is present now
    let html_page = app.get_login_page_html().await;
    assert!(html_page.contains("Invalid name"));

    // Check that error message is not present now
    let html_page = app.get_login_page_html().await;
    assert!(!html_page.contains("Invalid name"));
}

#[tokio::test]
async fn error_message_invalid_password_is_set_on_login() {
    let app = TestApp::spawn().await;

    let login_body = serde_json::json!({
        "name": "Hello",
        "password": ""
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to_resource(&response, "/login");

    // Check that error message is present now
    let html_page = app.get_login_page_html().await;
    assert!(html_page.contains("Invalid password"));

    // Check that error message is not present now
    let html_page = app.get_login_page_html().await;
    assert!(!html_page.contains("Invalid password"));
}

#[tokio::test]
async fn registration_works() {
    let app = TestApp::spawn().await;
    let register_body = serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass",
        "repeat_password": "!1Aapass"
    });

    let response = app.post_registration(&register_body).await;
    assert_is_redirect_to_resource(&response, "/blog_posts/all");
}

#[tokio::test]
async fn registration_logout_login_works() {
    let app = TestApp::spawn().await;
    let register_body = serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass",
        "repeat_password": "!1Aapass"
    });

    let response = app.post_registration(&register_body).await;
    assert_is_redirect_to_resource(&response, "/blog_posts/all");
    // Now we are logged

    let response = app.post_logout().await;
    assert_is_redirect_to_resource(&response, "/login");
    let login_page = app.get_login_page_html().await;
    assert!(login_page.contains("You have successfully logged out"));
    // Now we should be logged out

    let login_body = serde_json::json!({
        "name":  "SuperValidName",
        "password": "!1Aapass"
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to_resource(&response, "/blog_posts/all");
}

#[tokio::test]
async fn registration_with_invalid_password_and_not_equal_repeat_is_password_error() {
    let app = TestApp::spawn().await;

    let login_body = serde_json::json!({
        "name": "ValidName",
        "password": "aaaa",
        "repeat_password": "aaaa"
    });
    let response = app.post_registration(&login_body).await;

    assert_is_redirect_to_resource(&response, "/registration");

    let html_page = app.get_registration_page_html().await;
    assert!(html_page.contains("Invalid password"));

    let html_page = app.get_registration_page_html().await;
    assert!(!html_page.contains("Invalid password"));
}

#[tokio::test]
async fn you_cant_create_user_with_taken_name() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    test_user.register_internally(app.pool());

    let register_body = serde_json::json!({
        "name": test_user.name.as_ref(),
        "password": test_user.password.as_ref().expose_secret(),
        "repeat_password": test_user.password.as_ref().expose_secret()
    });

    let response = app.post_registration(&register_body).await;
    assert_is_redirect_to_resource(&response, "/registration");

    let html_page = app.get_registration_page_html().await;
    assert!(!html_page.contains("Taken name"));
}

#[tokio::test]
async fn try_to_get_login_page_after_login_is_redirect_to_account() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    test_user.register_internally(app.pool());
    test_user.login(&app).await;

    let response = app.get_login_page().await;
    assert_is_redirect_to_resource(&response, "/account/home");
}

#[tokio::test]
async fn try_to_get_registration_page_after_login_is_redirect_to_account() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    test_user.register_internally(app.pool());
    test_user.login(&app).await;

    let response = app.get_registration_page().await;
    assert_is_redirect_to_resource(&response, "/account/home");
}

#[tokio::test]
async fn user_name_persists_between_sequential_login_attempts() {
    let app = TestApp::spawn().await;

    let name = Uuid::new_v4().to_string();
    let login_body = serde_json::json!({
        "name": &name,
        "password": ""
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to_resource(&response, "/login");

    let html_page = app.get_login_page_html().await;
    assert!(html_page.contains(&name));

    let html_page = app.get_login_page_html().await;
    assert!(!html_page.contains(&name));
}

#[tokio::test]
async fn user_name_persists_between_sequential_registration_attempts() {
    let app = TestApp::spawn().await;

    let name = Uuid::new_v4().to_string();
    let login_body = serde_json::json!({
        "name": &name,
        "password": "",
        "repeat_password": ""
    });

    let response = app.post_registration(&login_body).await;
    assert_is_redirect_to_resource(&response, "/registration");

    let html_page = app.get_registration_page_html().await;
    assert!(html_page.contains(&name));

    let html_page = app.get_registration_page_html().await;
    assert!(!html_page.contains(&name));
}
