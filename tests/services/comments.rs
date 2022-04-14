use crate::helpers::{spawn_test_db, TestBlogPost, TestComment, TestUser};
use claim::{assert_ok, assert_some};
use holosite::domain::comments::{CommentID, NewComment, UpdateComment};
use holosite::services::{
    get_comment_by_id, get_comments_for_blog_post, get_comments_of_author, insert_new_comment,
    update_comment, Page,
};

#[test]
fn create_comment_works() {
    let pool = spawn_test_db();
    let user = TestUser::generate();
    let user_id = user.register_internally(&pool);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&pool, &user_id);
    let comment = TestComment::generate();

    let new_comment = NewComment {
        author_id: &user_id,
        post_id: &blog_post_id,
        parent_id: None,
        contents: &comment.contents,
    };
    let res = insert_new_comment(&pool, &new_comment);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.contents, comment.contents);
    assert_eq!(res.author_id, user_id);
    assert_eq!(res.post_id, blog_post_id);
}

#[test]
fn get_comment_by_id_works() {
    let pool = spawn_test_db();
    let user = TestUser::generate();
    let user_id = user.register_internally(&pool);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&pool, &user_id);
    let comment = TestComment::generate();
    let comment_id = comment.register_internally(&pool, &blog_post_id, &user_id);

    let res = get_comment_by_id(&pool, &comment_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.contents, comment.contents);
}

#[test]
fn get_comment_by_author_works() {
    let pool = spawn_test_db();
    let user = TestUser::generate();
    let user_id = user.register_internally(&pool);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&pool, &user_id);
    let comment = TestComment::generate();
    comment.register_internally(&pool, &blog_post_id, &user_id);

    let res = get_comments_of_author(&pool, &user_id, &Page::infinite());
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].contents, comment.contents);
}

#[test]
fn get_comment_by_author_returns_all_comments() {
    let pool = spawn_test_db();
    let user = TestUser::generate();
    let user_id = user.register_internally(&pool);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&pool, &user_id);

    let comment_ids: Vec<CommentID> = (0..100)
        .map(|_| {
            let comment = TestComment::generate();
            comment.register_internally(&pool, &blog_post_id, &user_id)
        })
        .collect();

    let res = get_comments_of_author(&pool, &user_id, &Page::infinite());
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.len(), comment_ids.len());
    for i in 0..res.len() {
        assert_eq!(res[i].id, comment_ids[i]);
    }
}

#[test]
fn get_comment_by_blog_post_works() {
    let pool = spawn_test_db();
    let user = TestUser::generate();
    let user_id = user.register_internally(&pool);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&pool, &user_id);
    let comment = TestComment::generate();
    comment.register_internally(&pool, &blog_post_id, &user_id);

    let res = get_comments_for_blog_post(&pool, &blog_post_id, &Page::infinite());
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].contents, comment.contents);
}

#[test]
fn get_comment_by_blog_post_returns_blog_posts_from_different_authors() {
    let pool = spawn_test_db();
    let user = TestUser::generate();
    let user_id = user.register_internally(&pool);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&pool, &user_id);

    let comment_ids: Vec<CommentID> = (0..100)
        .map(|_| {
            let user = TestUser::generate();
            let user_id = user.register_internally(&pool);
            let comment = TestComment::generate();
            comment.register_internally(&pool, &blog_post_id, &user_id)
        })
        .collect();

    let res = get_comments_for_blog_post(&pool, &blog_post_id, &Page::infinite());
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.len(), comment_ids.len());
    for i in 0..res.len() {
        assert_eq!(res[i].id, comment_ids[i]);
    }
}

#[test]
fn get_comment_by_blog_post_returns_all_comments() {
    let pool = spawn_test_db();
    let user = TestUser::generate();
    let user_id = user.register_internally(&pool);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&pool, &user_id);

    let comment_ids: Vec<CommentID> = (0..100)
        .map(|_| {
            let comment = TestComment::generate();
            comment.register_internally(&pool, &blog_post_id, &user_id)
        })
        .collect();

    let res = get_comments_for_blog_post(&pool, &blog_post_id, &Page::infinite());
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.len(), comment_ids.len());
    for i in 0..res.len() {
        assert_eq!(res[i].id, comment_ids[i]);
    }
}

#[test]
fn update_comment_contents_works() {
    let pool = spawn_test_db();
    let user = TestUser::generate();
    let user_id = user.register_internally(&pool);
    let blog_post = TestBlogPost::generate();
    let blog_post_id = blog_post.register_internally(&pool, &user_id);
    let comment = TestComment::generate();
    let comment_id = comment.register_internally(&pool, &blog_post_id, &user_id);

    let changeset = UpdateComment {
        id: &comment_id,
        contents: Some("New contents"),
        is_deleted: None,
    };
    let res = update_comment(&pool, &changeset);
    assert_ok!(&res);

    let res = get_comment_by_id(&pool, &comment_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.contents, "New contents");
}
