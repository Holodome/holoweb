use crate::common::{assert_is_redirect_to, TestApp};

#[tokio::test]
async fn account_shows_correct_user_name() {
    let app = TestApp::spawn().await;
    let register_body = serde_json::json!({
        "name": "SuperValidName",
        "password": "!1Aapass",
        "repeat_password": "!1Aapass"
    });

    let response = app.post_registration(&register_body).await;
    assert_is_redirect_to(&response, "/");
    // Now we are logged

    let response = app.get_account_page_html().await;
    assert!(response.contains("SuperValidName"));
}
