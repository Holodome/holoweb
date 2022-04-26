use crate::common::{TestApp, TestBlogPost, TestComment, TestUser};

#[tokio::test]
async fn test_user_page_works() {
    let app = TestApp::spawn().await;
    let user = TestUser::generate();
    let user_id = user.register_internally(app.pool());

    let html = app.get_user_page_html(user_id.as_ref()).await;
    assert!(html.contains(user.name.as_ref()))
}

#[tokio::test]
async fn test_user_page_displays_blog_post() {
    let app = TestApp::spawn().await;
    let user = TestUser::generate();
    let user_id = user.register_internally(app.pool());

    let blog_post = TestBlogPost::generate();
    blog_post.register_internally(app.pool(), &user_id);

    let html = app.get_user_page_html(user_id.as_ref()).await;
    assert!(html.contains(&blog_post.title))
}

#[tokio::test]
async fn test_user_page_displays_comment() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);
    let test_comment = TestComment::generate();
    test_comment.register_internally(app.pool(), &blog_post_id, &user_id);

    let html = app.get_user_page_html(user_id.as_ref()).await;
    assert!(html.contains(&test_comment.contents))
}
