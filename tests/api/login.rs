use crate::helpers::{assert_is_redirect_to, TestApp};

#[actix_web::test]
async fn logout_returns_redirect_to_login_when_not_logged_in() {
    let app = TestApp::spawn().await;
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login")
}

#[actix_web::test]
async fn error_flash_message_is_set_on_failure() {
    let app = TestApp::spawn().await;

    let login_body = serde_json::json!({
        "name": "some name",
        "password": "some password"
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to(&response, "/login");

    let html_page = app.get_login_page_html().await;
    assert!(html_page.contains("Authentication failed"));

    let html_page = app.get_login_page_html().await;
    assert!(!html_page.contains("Authentication failed"));
}