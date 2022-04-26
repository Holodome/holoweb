use crate::api::assert_is_redirect_to_resource;
use crate::common::{extract_csrf_token, TestApp, TestUser};
use secrecy::ExposeSecret;
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_change_password() {
    let app = TestApp::spawn().await;
    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    app.post_logout().await;

    let new_password = Uuid::new_v4().to_string();
    let response = app
        .post_change_password(&serde_json::json!({
            "csrf_token": csrf,
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "repeat_new_password": &new_password
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    let response = app
        .post_change_password(&serde_json::json!({
            "csrf_token": csrf,
            "current_password": user.password.as_ref().expose_secret(),
            "new_password": &new_password,
            "repeat_new_password": &another_new_password
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/account/home");

    let html = app.get_account_page_html().await;
    assert!(html.contains("Repeat password does not match new password"));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    let response = app
        .post_change_password(&serde_json::json!({
            "csrf_token": csrf,
            "current_password": &wrong_password,
            "new_password": &new_password,
            "repeat_new_password": &new_password
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/account/home");

    let html = app.get_account_page_html().await;
    assert!(html.contains("Current password is incorrect"));
}

#[tokio::test]
async fn new_password_must_be_valid() {
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();

    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    let response = app
        .post_change_password(&serde_json::json!({
            "csrf_token": csrf,
            "current_password": user.password.as_ref().expose_secret(),
            "new_password": &new_password,
            "repeat_new_password": &new_password
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/account/home");

    let html = app.get_account_page_html().await;
    assert!(html.contains("New password is invalid"));
}

#[tokio::test]
async fn change_password_works() {
    let app = TestApp::spawn().await;
    let new_password = "!1Aaaaaa";

    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    let response = app
        .post_change_password(&serde_json::json!({
            "csrf_token": csrf,
            "current_password": user.password.as_ref().expose_secret(),
            "new_password": &new_password,
            "repeat_new_password": &new_password
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/account/home");

    let html = app.get_account_page_html().await;
    assert!(html.contains("Your password has been changed"));

    let response = app.post_logout().await;
    assert_is_redirect_to_resource(&response, "/login");

    let html = app.get_login_page_html().await;
    assert!(html.contains("You have successfully logged out"));

    let response = app
        .post_login(&serde_json::json!({
            "name": user.name.as_ref(),
            "password": &new_password
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/blog_posts/all");
}
