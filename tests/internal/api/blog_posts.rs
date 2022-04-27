use crate::api::{assert_is_redirect_to_resource, assert_resp_ok};
use crate::common::{extract_csrf_token, TestApp, TestBlogPost, TestComment, TestUser};
use holosite::domain::blog_posts::BlogPostVisibility;

#[tokio::test]
async fn you_must_be_logged_in_to_see_create_blog_post_page() {
    let app = TestApp::spawn().await;
    let response = app.get_create_blog_post_page().await;
    assert_is_redirect_to_resource(&response, "/login");

    let test_user = TestUser::generate();
    test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let response = app.get_create_blog_post_page().await;
    assert_resp_ok(&response);
}

#[tokio::test]
async fn you_are_not_required_to_be_logged_in_to_see_all_blog_posts() {
    let app = TestApp::spawn().await;
    let response = app.get_all_blog_posts_page().await;
    assert_resp_ok(&response);
}

#[tokio::test]
async fn create_blog_post_and_see_it_appears_at_dashboard() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    test_user.register_internally(app.pool());
    test_user.login(&app).await;

    let blog_post = TestBlogPost::generate();
    let csrf = extract_csrf_token(&app.get_create_blog_post_page_html().await);
    app.post_create_blog_post(&blog_post.to_json(&csrf)).await;

    let html = app.get_all_blog_posts_page_html().await;
    assert!(html.contains(&blog_post.title));
}

#[tokio::test]
async fn you_must_be_logged_in_to_create_blog_post() {
    let app = TestApp::spawn().await;
    let response = app.get_create_blog_post_page().await;
    assert_is_redirect_to_resource(&response, "/login");
}

#[tokio::test]
async fn view_blog_post_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);

    let response = app
        .get_view_blog_post_page(blog_post_id.as_ref().as_str())
        .await;
    assert_resp_ok(&response);
    let html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
        .await;
    assert!(html.contains(&blog_post.title))
}

#[tokio::test]
async fn you_must_be_logged_in_to_edit_blog_post() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);

    let response = app.get_edit_blog_post_page(blog_post_id.as_ref()).await;
    assert_is_redirect_to_resource(&response, "/login");

    test_user.login(&app).await;
    let response = app.get_edit_blog_post_page(blog_post_id.as_ref()).await;
    assert_resp_ok(&response);
}

#[tokio::test]
async fn edit_blog_post_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);

    let response = app
        .get_edit_blog_post_page(blog_post_id.as_ref().as_str())
        .await;
    assert_resp_ok(&response);
    let html = app
        .get_edit_blog_post_page_html(blog_post_id.as_ref().as_str())
        .await;
    assert!(html.contains(&blog_post.title));

    let csrf = extract_csrf_token(
        &app.get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
            .await,
    );

    let updated = TestBlogPost::generate();
    app.post_edit_blog_post(&updated.to_json(&csrf), &blog_post_id)
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

#[tokio::test]
async fn you_must_be_logged_in_to_leave_comments() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);

    let csrf = extract_csrf_token(
        &app.get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
            .await,
    );

    let comment = TestComment::generate();

    let response = app
        .post_create_comment(
            &serde_json::json!({
                "csrf_token": csrf,
                "contents": comment.contents
            }),
            &blog_post_id,
        )
        .await;
    assert_is_redirect_to_resource(&response, "/login");
}

#[tokio::test]
async fn cant_view_protected_blog_post() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());

    let blog_post = TestBlogPost::generate_authenticated();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);

    let response = app.get_view_blog_post_page(blog_post_id.as_ref()).await;
    assert_resp_ok(&response);

    let html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(html.contains("You have to be authenticated to view this blog post"))
}

#[tokio::test]
async fn can_change_blog_post_visibility() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());

    let mut blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);

    let html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(!html.contains("You have to be authenticated to view this blog post"));

    blog_post.visibility = BlogPostVisibility::Authenticated;
    test_user.login(&app).await;
    let csrf = extract_csrf_token(
        &app.get_edit_blog_post_page_html(blog_post_id.as_ref().as_str())
            .await,
    );
    app.post_edit_blog_post(&blog_post.to_json(&csrf), &blog_post_id)
        .await;
    app.post_logout().await;

    let html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(html.contains("You have to be authenticated to view this blog post"))
}

#[tokio::test]
async fn blogpost_contents_are_rendered_from_markdown() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;

    let mut blog_post = TestBlogPost::generate();
    blog_post.contents = r#"
# This is title

*Hello world*

`inline code`
    "#
    .to_string();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);

    let response = app
        .get_view_blog_post_page(blog_post_id.as_ref().as_str())
        .await;
    assert_resp_ok(&response);
    let html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
        .await;
    assert!(html.contains(&blog_post.title));
    assert!(html.contains("<h1>This is title</h1>"));
    assert!(html.contains("<em>Hello world</em>"));
    assert!(html.contains("<p><code>inline code</code></p>"));
}
