use crate::api::assert_is_redirect_to;
use crate::common::TestApp;

#[tokio::test]
async fn test_home_is_redirect_to_blog_posts() {
    let app = TestApp::spawn().await;
    let response = app.get_page("/").await;
    assert_is_redirect_to(&response, "/blog_posts/all");
}
