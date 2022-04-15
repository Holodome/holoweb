use crate::helpers::{
    assert_is_redirect_to, assert_resp_forbidden, TestApp, TestBlogPost, TestComment, TestUser,
};

#[tokio::test]
async fn create_comment_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&app.db.pool);
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&app.db.pool, &user_id);
    let test_comment = TestComment::generate();

    let response = app
        .post_create_comment(
            &serde_json::json!({
                "contents": &test_comment.contents
            }),
            &blog_post_id,
        )
        .await;

    assert_is_redirect_to(&response, &format!("/blog_posts/{}", blog_post_id.as_ref()));

    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(post_html.contains(&test_comment.contents));
}

#[tokio::test]
async fn edit_comment_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&app.db.pool);
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&app.db.pool, &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(&app.db.pool, &blog_post_id, &user_id);

    let response = app
        .post_edit_comment(
            &serde_json::json!({
                "contents": "New contents",
                "is_deleted": false
            }),
            &blog_post_id,
            &comment_id,
        )
        .await;
    assert_is_redirect_to(&response, &format!("/blog_posts/{}", blog_post_id.as_ref()));

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
    let user_id = test_user.register_internally(&app.db.pool);
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&app.db.pool, &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(&app.db.pool, &blog_post_id, &user_id);

    let other_user = TestUser::generate();
    other_user.register_internally(&app.db.pool);
    other_user.login(&app).await;

    let response = app
        .post_edit_comment(
            &serde_json::json!({
                "contents": "New contents",
                "is_deleted": false
            }),
            &blog_post_id,
            &comment_id,
        )
        .await;
    assert_resp_forbidden(&response);

    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;
    assert!(!post_html.contains("New contents"));
    assert!(post_html.contains(&test_comment.contents));
}

#[tokio::test]
async fn delete_comment_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&app.db.pool);
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&app.db.pool, &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(&app.db.pool, &blog_post_id, &user_id);

    let response = app
        .post_edit_comment(
            &serde_json::json!({
                "contents": "New contents",
                "is_deleted": true
            }),
            &blog_post_id,
            &comment_id,
        )
        .await;
    assert_is_redirect_to(&response, &format!("/blog_posts/{}", blog_post_id.as_ref()));

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
    let user_id = test_user.register_internally(&app.db.pool);
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&app.db.pool, &user_id);
    let comment_contents: Vec<String> = (0..100)
        .map(|_| {
            let comment = TestComment::generate();
            comment.register_internally(&app.db.pool, &blog_post_id, &user_id);
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
    let user_id = test_user.register_internally(&app.db.pool);
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&app.db.pool, &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(&app.db.pool, &blog_post_id, &user_id);
    let other_comment = TestComment::generate();

    let response = app
        .post_create_comment(
            &serde_json::json!({
                "contents": &other_comment.contents,
                "parent": comment_id.as_ref()
            }),
            &blog_post_id,
        )
        .await;

    assert_is_redirect_to(&response, &format!("/blog_posts/{}", blog_post_id.as_ref()));
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
    let user_id = test_user.register_internally(&app.db.pool);
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&app.db.pool, &user_id);
    let test_comment = TestComment::generate();
    let comment_id = test_comment.register_internally(&app.db.pool, &blog_post_id, &user_id);
    let response_comment = TestComment::generate();
    response_comment.register_response_internally(
        &app.db.pool,
        &blog_post_id,
        &user_id,
        &comment_id,
    );

    let response = app
        .post_edit_comment(
            &serde_json::json!({
                "contents": "New contents",
                "is_deleted": true
            }),
            &blog_post_id,
            &comment_id,
        )
        .await;
    assert_is_redirect_to(&response, &format!("/blog_posts/{}", blog_post_id.as_ref()));
    let post_html = app
        .get_view_blog_post_page_html(blog_post_id.as_ref())
        .await;

    assert!(post_html.contains(&response_comment.contents));
    assert!(!post_html.contains(&test_comment.contents));
}
