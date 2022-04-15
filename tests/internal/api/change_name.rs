use crate::api::assert_is_redirect_to;
use crate::common::{TestApp, TestUser};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_change_name_form() {
    let app = TestApp::spawn().await;
    let response = app.get_change_name().await;
    assert_is_redirect_to(&response, "/login");

    let test_user = TestUser::generate();
    test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let response = app.get_change_name().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_password() {
    let app = TestApp::spawn().await;

    let response = app
        .post_change_name(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_name": "Hello",
        }))
        .await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn password_must_be_valid() {
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
            "current_password": Uuid::new_v4().to_string(),
            "new_name": "Hello",
        }))
        .await;
    assert_is_redirect_to(&response, "/change_name");

    let html = app.get_change_name_page_html().await;
    assert!(html.contains("Current password is invalid"));
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
            "current_password": "!1Aapass",
            "new_name": "",
        }))
        .await;
    assert_is_redirect_to(&response, "/change_name");

    let html = app.get_change_name_page_html().await;
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
            "current_password": "!1Aapass",
            "new_name": "TakenName",
        }))
        .await;
    assert_is_redirect_to(&response, "/change_name");

    let html = app.get_change_name_page_html().await;
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
            "current_password": "!1Aapass",
            "new_name": "NewName",
        }))
        .await;
    assert_is_redirect_to(&response, "/account");

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
