use crate::common::{assert_is_redirect_to, TestApp, TestUser};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_change_password_form() {
    let app = TestApp::spawn().await;
    let response = app.get_change_password().await;
    assert_is_redirect_to(&response, "/login");

    let test_user = TestUser::generate();
    test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let response = app.get_change_password().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_password() {
    let app = TestApp::spawn().await;

    let new_password = Uuid::new_v4().to_string();
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "repeat_new_password": &new_password
        }))
        .await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    app.post_registration(&serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass",
        "repeat_password": "!1Aapass"
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": "!1Aapass",
            "new_password": &new_password,
            "repeat_new_password": &another_new_password
        }))
        .await;
    assert_is_redirect_to(&response, "/change_password");

    let html = app.get_change_password_page_html().await;
    assert!(html.contains("Repeat password does not match new password"));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    app.post_registration(&serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass",
        "repeat_password": "!1Aapass"
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "repeat_new_password": &new_password
        }))
        .await;
    assert_is_redirect_to(&response, "/change_password");

    let html = app.get_change_password_page_html().await;
    assert!(html.contains("Current password is incorrect"));
}

#[tokio::test]
async fn new_password_must_be_valid() {
    let app = TestApp::spawn().await;
    let new_password = Uuid::new_v4().to_string();

    app.post_registration(&serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass",
        "repeat_password": "!1Aapass"
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": "!1Aapass",
            "new_password": &new_password,
            "repeat_new_password": &new_password
        }))
        .await;
    assert_is_redirect_to(&response, "/change_password");

    let html = app.get_change_password_page_html().await;
    assert!(html.contains("New password is invalid"));
}

#[tokio::test]
async fn change_password_works() {
    let app = TestApp::spawn().await;
    let new_password = "!1Aaaaaa";

    app.post_registration(&serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass",
        "repeat_password": "!1Aapass"
    }))
    .await;

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": "!1Aapass",
            "new_password": &new_password,
            "repeat_new_password": &new_password
        }))
        .await;
    assert_is_redirect_to(&response, "/account");

    let html = app.get_change_password_page_html().await;
    assert!(html.contains("Your password has been changed"));

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let html = app.get_login_page_html().await;
    assert!(html.contains("You have successfully logged out"));

    let response = app
        .post_login(&serde_json::json!({
            "name": "SuperValidName",
            "password": &new_password
        }))
        .await;
    assert_is_redirect_to(&response, "/");
}
