use crate::helpers::{assert_is_redirect_to, spawn_app};

#[actix_web::test]
async fn logout_returns_redirect_to_login_when_not_logged_in() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(&format!("{}/logout", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert_is_redirect_to(&response, "/login")
}
