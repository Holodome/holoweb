use crate::helpers::{assert_is_redirect_to, TestApp};

#[tokio::test]
async fn logout_returns_redirect_to_login_when_not_logged_in() {
    let app = TestApp::spawn().await;
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login")
}

#[tokio::test]
async fn error_flash_message_is_set_on_failure() {
    let app = TestApp::spawn().await;

    let login_body = serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass"
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to(&response, "/login");

    // Check that error message is present now
    let html_page = app.get_login_page_html().await;
    assert!(html_page.contains("Authentication failed"));

    // Check that error message is not present now
    let html_page = app.get_login_page_html().await;
    assert!(!html_page.contains("Authentication failed"));
}

#[tokio::test]
async fn error_message_invalid_credentials_is_set_on_login() {
    let app = TestApp::spawn().await;

    let login_body = serde_json::json!({
        "name": "1",
        "password": "1"
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to(&response, "/login");

    // Check that error message is present now
    let html_page = app.get_login_page_html().await;
    assert!(html_page.contains("Invalid credentials"));

    // Check that error message is not present now
    let html_page = app.get_login_page_html().await;
    assert!(!html_page.contains("Invalid credentials"));
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
    assert_is_redirect_to(&response, "/");
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
    assert_is_redirect_to(&response, "/");
    // Now we are logged

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");
    let login_page = app.get_login_page_html().await;
    assert!(login_page.contains("You have successfully logged out"));
    // Now we should be logged out

    let login_body = serde_json::json!({
        "name":  "SuperValidName",
        "password": "!1Aapass"
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/");
}
