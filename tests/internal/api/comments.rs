use crate::api::{assert_is_redirect_to_resource, assert_resp_forbidden};
use crate::common::{extract_csrf_token, TestApp, TestBlogPost, TestComment, TestUser};
use actix_web::web::post;

#[tokio::test]
async fn create_comment_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);
    let test_comment = TestComment::generate();

    let csrf = extract_csrf_token(
        &app.get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
            .await,
    );

    let response = app
        .post_create_comment(
            &serde_json::json!({
                "csrf_token": csrf,
                "contents": &test_comment.contents
            }),
            &blog_post_id,
        )
        .await;

    assert_is_redirect_to_resource(
        &response,
        &format!("/blog_posts/{}/view", blog_post_id.as_ref()),
    );

    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(post_html.contains(&test_comment.contents));
}

#[tokio::test]
async fn edit_comment_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(app.pool(), &blog_post_id, &user_id);

    let csrf = extract_csrf_token(
        &app.get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
            .await,
    );

    let response = app
        .post_edit_comment(
            &serde_json::json!({
                "csrf_token": csrf,
                "contents": "New contents",
                "is_deleted": false
            }),
            &blog_post_id,
            &comment_id,
        )
        .await;
    assert_is_redirect_to_resource(
        &response,
        &format!("/blog_posts/{}/view", blog_post_id.as_ref()),
    );

    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(post_html.contains("New contents"));
    assert!(!post_html.contains(&test_comment.contents));
}

#[tokio::test]
async fn cant_edit_others_comment() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;

    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(app.pool(), &blog_post_id, &user_id);

    app.post_logout().await;

    let other_user = TestUser::generate();
    other_user.register_internally(app.pool());
    other_user.login(&app).await;

    let csrf = extract_csrf_token(
        &app.get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
            .await,
    );

    let response = app
        .post_edit_comment(
            &serde_json::json!({
                "csrf_token": csrf,
                "contents": "New contents",
            }),
            &blog_post_id,
            &comment_id,
        )
        .await;

    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(!post_html.contains("New contents"));
    assert!(post_html.contains("Can't change others comment"));
    assert!(post_html.contains(&test_comment.contents));
}

#[tokio::test]
async fn delete_comment_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(app.pool(), &blog_post_id, &user_id);

    let response = app.post_delete_comment(&blog_post_id, &comment_id).await;
    assert_is_redirect_to_resource(
        &response,
        &format!("/blog_posts/{}/view", blog_post_id.as_ref()),
    );

    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(!post_html.contains("New contents"));
    assert!(!post_html.contains(&test_comment.contents));
}

#[tokio::test]
async fn all_comments_for_blog_post_are_displayed_correctly() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);
    let comment_contents: Vec<String> = (0..100)
        .map(|_| {
            let comment = TestComment::generate();
            comment.register_internally(app.pool(), &blog_post_id, &user_id);
            comment.contents
        })
        .collect();

    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    for i in 0..comment_contents.len() {
        assert!(post_html.contains(&comment_contents[i]));
    }
}

#[tokio::test]
async fn create_response_comment_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(app.pool(), &blog_post_id, &user_id);
    let other_comment = TestComment::generate();

    let csrf = extract_csrf_token(
        &app.get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
            .await,
    );

    let response = app
        .post_create_comment(
            &serde_json::json!({
                "csrf_token": csrf,
                "contents": &other_comment.contents,
                "parent": comment_id.as_ref()
            }),
            &blog_post_id,
        )
        .await;

    assert_is_redirect_to_resource(
        &response,
        &format!("/blog_posts/{}/view", blog_post_id.as_ref()),
    );
    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(post_html.contains(&other_comment.contents));
    assert!(post_html.contains(&test_comment.contents));
}

#[tokio::test]
async fn delete_comment_in_middle_of_response_tree_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(app.pool());
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(app.pool(), &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(app.pool(), &blog_post_id, &user_id);
    let response_comment = TestComment::generate();
    response_comment.register_response_internally(app.pool(), &blog_post_id, &user_id, &comment_id);

    let csrf = extract_csrf_token(
        &app.get_view_blog_post_page_html(blog_post_id.as_ref().as_str())
            .await,
    );

    let response = app
        .post_edit_comment(
            &serde_json::json!({
                "csrf_token": csrf,
                "contents": "New contents",
                "is_deleted": true
            }),
            &blog_post_id,
            &comment_id,
        )
        .await;
    assert_is_redirect_to_resource(
        &response,
        &format!("/blog_posts/{}/view", blog_post_id.as_ref()),
    );
    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;

    assert!(post_html.contains(&response_comment.contents));
    assert!(!post_html.contains(&test_comment.contents));
}
