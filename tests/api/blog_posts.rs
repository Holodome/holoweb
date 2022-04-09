use crate::helpers::{assert_is_redirect_to, TestApp, TestBlogPost, TestUser};

#[tokio::test]
async fn you_must_be_logged_in_to_see_create_blog_post_page() {
    let app = TestApp::spawn().await;
    let response = app.get_create_blog_post_page().await;
    assert_is_redirect_to(&response, "/login");

    let test_user = TestUser::generate();
    test_user.register_internally(&app).await;
    test_user.login(&app).await;
    let response = app.get_create_blog_post_page().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn you_are_not_required_to_be_logged_in_to_see_all_blog_posts() {
    let app = TestApp::spawn().await;
    let response = app.get_all_blog_posts_page().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn create_blog_post_and_see_it_appears_at_dashboard() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    test_user.register_internally(&app).await;
    test_user.login(&app).await;

    let blog_post = TestBlogPost::generate();
    let response = app.post_create_blog_post(&blog_post.to_json()).await;
    assert_is_redirect_to(&response, "/blog_post/all");

    let html = app.get_all_blog_posts_page_html().await;
    assert!(html.contains(&blog_post.title));
}

#[tokio::test]
async fn you_must_be_logged_in_to_create_blog_post() {
    let app = TestApp::spawn().await;
    let blog_post = TestBlogPost::generate();
    let response = app.post_create_blog_post(&blog_post.to_json()).await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn view_blog_post_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&app).await;
    test_user.login(&app).await;

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.create_internally(&app, &user_id);

    let response = app
        .get_view_blog_post_page(blog_post_id.as_ref().as_str())
        .await;
    assert_eq!(response.status(), 200);
    let html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
        .await;
    assert!(html.contains(&blog_post.title))
}

#[tokio::test]
async fn you_must_be_logged_in_to_edit_blog_post() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&app).await;

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.create_internally(&app, &user_id);

    let response = app.get_edit_blog_post_page(&blog_post_id.as_ref()).await;
    assert_is_redirect_to(&response, "/login");

    test_user.login(&app).await;
    let response = app.get_edit_blog_post_page(&blog_post_id.as_ref()).await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn edit_blog_post_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&app).await;
    test_user.login(&app).await;

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.create_internally(&app, &user_id);

    let response = app
        .get_edit_blog_post_page(blog_post_id.as_ref().as_str())
        .await;
    assert_eq!(response.status(), 200);
    let html = app
        .get_edit_blog_post_page_html(blog_post_id.as_ref().as_str())
        .await;
    assert!(html.contains(&blog_post.title));

    let updated = TestBlogPost::generate();
    app.post_edit_blog_post(&updated.to_json(), &blog_post_id)
        .await;

    let response = app
        .get_view_blog_post_page(blog_post_id.as_ref().as_str())
        .await;
    assert_eq!(response.status(), 200);
    let html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
        .await;
    assert!(html.contains(&updated.title))
}
