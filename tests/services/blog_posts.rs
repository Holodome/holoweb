use crate::helpers::{get_db_connection, TestBlogPost, TestUser};
use claim::{assert_ok, assert_some};
use holosite::domain::blog_posts::{BlogPostID, NewBlogPost, UpdateBlogPost};
use holosite::services::{
    get_all_blog_posts, get_blog_post_by_id, get_blog_post_by_title, insert_new_blog_post,
    update_blog_post, Page,
};

#[test]
fn add_new_blog_post_works() {
    let pool = get_db_connection();
    let test_post = TestBlogPost::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&pool);

    let res = insert_new_blog_post(
        &pool,
        &NewBlogPost {
            title: &test_post.title,
            brief: &test_post.brief,
            contents: &test_post.contents,
            author_id: &user_id,
        },
    );
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_post.title);
    assert_eq!(res.brief, test_post.brief);
    assert_eq!(res.contents, test_post.contents);
}

#[test]
fn add_blog_post_and_get_it_by_title_works() {
    let pool = get_db_connection();
    let test_post = TestBlogPost::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&pool);
    let post_id = test_post.register_internally(&pool, &user_id);

    let res = get_blog_post_by_id(&pool, &post_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_post.title);
    assert_eq!(res.brief, test_post.brief);
    assert_eq!(res.contents, test_post.contents);
}

#[test]
fn add_blog_post_and_get_it_by_id_works() {
    let pool = get_db_connection();
    let test_post = TestBlogPost::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&pool);
    test_post.register_internally(&pool, &user_id);

    let res = get_blog_post_by_title(&pool, &test_post.title);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_post.title);
    assert_eq!(res.brief, test_post.brief);
    assert_eq!(res.contents, test_post.contents);
}

#[test]
fn update_blog_post_title_works() {
    let pool = get_db_connection();
    let test_post = TestBlogPost::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&pool);
    let post_id = test_post.register_internally(&pool, &user_id);

    let changeset = UpdateBlogPost {
        id: &post_id,
        title: Some(&"New Title"),
        brief: None,
        contents: None,
    };
    let res = update_blog_post(&pool, &changeset);
    assert_ok!(res);

    let res = get_blog_post_by_id(&pool, &post_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, "New Title");
    assert_eq!(res.brief, test_post.brief);
    assert_eq!(res.contents, test_post.contents);
}

#[test]
fn cant_change_blog_post_title_to_taken_name() {}

#[test]
fn change_blog_post_brief_works() {
    let pool = get_db_connection();
    let test_post = TestBlogPost::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&pool);
    let post_id = test_post.register_internally(&pool, &user_id);

    let changeset = UpdateBlogPost {
        id: &post_id,
        title: None,
        brief: Some(&"New Brief"),
        contents: None,
    };
    let res = update_blog_post(&pool, &changeset);
    assert_ok!(res);

    let res = get_blog_post_by_id(&pool, &post_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_post.title);
    assert_eq!(res.brief, "New Brief");
    assert_eq!(res.contents, test_post.contents);
}

#[test]
fn change_blog_post_contents_works() {
    let pool = get_db_connection();
    let test_post = TestBlogPost::generate();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&pool);
    let post_id = test_post.register_internally(&pool, &user_id);

    let changeset = UpdateBlogPost {
        id: &post_id,
        title: None,
        brief: None,
        contents: Some(&"New contents"),
    };
    let res = update_blog_post(&pool, &changeset);
    assert_ok!(res);

    let res = get_blog_post_by_id(&pool, &post_id);
    assert_ok!(&res);
    let res = res.unwrap();
    assert_some!(&res);
    let res = res.unwrap();
    assert_eq!(res.title, test_post.title);
    assert_eq!(res.brief, test_post.brief);
    assert_eq!(res.contents, "New contents");
}

#[test]
fn cant_add_new_blog_post_with_taken_title() {}

#[test]
fn get_all_blog_posts_works() {
    let pool = get_db_connection();
    let test_user = TestUser::generate();
    let user_id = test_user.register_internally(&pool);
    let post_ids: Vec<BlogPostID> = (0..100)
        .map(|_| {
            let test_post = TestBlogPost::generate();
            test_post.register_internally(&pool, &user_id)
        })
        .collect();

    let res = get_all_blog_posts(&pool, &Page::infinite());
    assert_ok!(&res);
    let res = res.unwrap();
    assert_eq!(res.len(), post_ids.len());
    for i in 0..res.len() {
        assert_eq!(res[i].id, post_ids[i]);
    }
}
