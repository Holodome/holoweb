use crate::helpers::{assert_is_redirect_to, TestApp};

#[actix_web::test]
async fn logout_returns_redirect_to_login_when_not_logged_in() {
    let app = TestApp::spawn().await;
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login")
}
