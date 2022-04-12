use crate::helpers::{assert_is_redirect_to, TestApp, TestBlogPost, TestComment, TestUser};

#[tokio::test]
async fn create_comment_works() {
    let app = TestApp::spawn().await;
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&app.pool);
    test_user.login(&app).await;
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&app.pool, &user_id);
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
}

#[tokio::test]
async fn edit_comment_works() {
    todo!()
}

#[tokio::test]
async fn cant_edit_others_comment() {
    todo!()
}

#[tokio::test]
async fn delete_comment_works() {
    todo!()
}

#[tokio::test]
async fn all_comments_for_blog_post_are_displayed_correctly() {
    todo!()
}

#[tokio::test]
async fn create_response_comment_works() {
    todo!()
}

#[tokio::test]
async fn displaying_response_tree_works() {
    todo!()
}

#[tokio::test]
async fn delete_comment_in_middle_of_response_tree_works() {
    todo!()
}
