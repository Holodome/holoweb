use crate::common::{TestApp, TestUser};

#[tokio::test]
async fn test_user_page_works() {
    let app = TestApp::spawn().await;
    let user = TestUser::generate();
    let user_id = user.register_internally(app.pool());

    let html = app.get_user_page_html(user_id.as_ref()).await;
    assert!(html.contains(user.name.as_ref()))
}
