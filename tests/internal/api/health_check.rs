use crate::common::TestApp;

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;
    let response = app.get_health_check().await;
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
