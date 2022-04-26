use crate::api::assert_is_redirect_to_resource;
use crate::common::{extract_csrf_token, TestApp, TestUser};
use secrecy::ExposeSecret;

#[tokio::test]
async fn you_must_be_logged_in_to_change_password() {
    let app = TestApp::spawn().await;
    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    app.post_logout().await;

    let response = app
        .post_change_name(&serde_json::json!({
            "csrf_token": csrf,
            "new_name": "Hello",
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/login");
}

#[tokio::test]
async fn new_name_must_be_valid() {
    let app = TestApp::spawn().await;
    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    let response = app
        .post_change_name(&serde_json::json!({
            "csrf_token": csrf,
            "new_name": "",
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/account/home");

    let html = app.get_account_page_html().await;
    assert!(html.contains(user.name.as_ref()));
}

#[tokio::test]
async fn cant_change_to_taken_name() {
    let app = TestApp::spawn().await;
    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let other_user = TestUser::generate();
    other_user.register_internally(app.pool());

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    let response = app
        .post_change_name(&serde_json::json!({
            "csrf_token": csrf,
            "new_name": other_user.name.as_ref(),
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/account/home");

    let html = app.get_account_page_html().await;
    assert!(html.contains(user.name.as_ref()));
}

#[tokio::test]
async fn change_name_works() {
    let app = TestApp::spawn().await;
    let user = TestUser::generate();
    user.register_internally(app.pool());
    user.login(&app).await;

    let account = app.get_account_page_html().await;
    let csrf = extract_csrf_token(&account);

    let response = app
        .post_change_name(&serde_json::json!({
            "csrf_token": csrf,
            "new_name": "NewName",
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/account/home");

    let response = app.post_logout().await;
    assert_is_redirect_to_resource(&response, "/login");

    let html = app.get_login_page_html().await;
    assert!(html.contains("You have successfully logged out"));

    let response = app
        .post_login(&serde_json::json!({
            "name": "NewName",
            "password": user.password.as_ref().expose_secret()
        }))
        .await;
    assert_is_redirect_to_resource(&response, "/blog_posts/all");
}
