use crate::api::assert_is_redirect_to;
use crate::common::TestApp;

#[tokio::test]
async fn you_must_be_logged_in_to_change_password() {
    let app = TestApp::spawn().await;

    let response = app
        .post_change_name(&serde_json::json!({
            "new_name": "Hello",
        }))
        .await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_name_must_be_valid() {
    let app = TestApp::spawn().await;

    let response = app
        .post_registration(&serde_json::json!({
            "name": "SuperValidName",
            "password": "!1Aapass",
            "repeat_password": "!1Aapass"
        }))
        .await;
    assert_is_redirect_to(&response, "/");

    let response = app
        .post_change_name(&serde_json::json!({
            "new_name": "",
        }))
        .await;
    assert_is_redirect_to(&response, "/account/home");

    let html = app.get_account_page_html().await;
    assert!(html.contains("Invalid name"));
}

#[tokio::test]
async fn cant_change_to_taken_name() {
    let app = TestApp::spawn().await;

    let response = app
        .post_registration(&serde_json::json!({
            "name": "TakenName",
            "password": "!1Aapass",
            "repeat_password": "!1Aapass"
        }))
        .await;
    assert_is_redirect_to(&response, "/");

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let response = app
        .post_registration(&serde_json::json!({
            "name": "SuperValidName",
            "password": "!1Aapass",
            "repeat_password": "!1Aapass"
        }))
        .await;
    assert_is_redirect_to(&response, "/");

    let response = app
        .post_change_name(&serde_json::json!({
            "new_name": "TakenName",
        }))
        .await;
    assert_is_redirect_to(&response, "/account/home");

    let html = app.get_account_page_html().await;
    assert!(html.contains("Taken name"));
}

#[tokio::test]
async fn change_name_works() {
    let app = TestApp::spawn().await;

    let response = app
        .post_registration(&serde_json::json!({
            "name": "TakenName",
            "password": "!1Aapass",
            "repeat_password": "!1Aapass"
        }))
        .await;
    assert_is_redirect_to(&response, "/");

    let response = app
        .post_change_name(&serde_json::json!({
            "new_name": "NewName",
        }))
        .await;
    assert_is_redirect_to(&response, "/account/home");

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let html = app.get_login_page_html().await;
    assert!(html.contains("You have successfully logged out"));

    let response = app
        .post_login(&serde_json::json!({
            "name": "NewName",
            "password": "!1Aapass"
        }))
        .await;
    assert_is_redirect_to(&response, "/");
}
