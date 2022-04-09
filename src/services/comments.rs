use crate::domain::comments::{Comment, CommentID, NewComment, UpdateComment};
use crate::schema::comments::dsl::*;
use crate::startup::Pool;
use diesel::{insert_into, update, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use crate::domain::blog_posts::BlogPostID;
use crate::domain::users::UserID;
use crate::services::Page;

pub fn get_comment_by_id(
    pool: &Pool,
    comment_id: &CommentID
) -> Result<Option<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(id.eq(comment_id))
        .first::<Comment>(&conn)
        .optional()?)
}

pub fn get_comments_of_author(
    pool: &Pool,
    author_id: &UserID,
    page: &Page
) -> Result<Vec<Comment>, anyhow::Error> {
    todo!()
}

pub fn get_comments_for_blog_post(
    pool: &Pool,
    post_id: &BlogPostID,
    page: &Page
) -> Result<Vec<Comment>, anyhow::Error> {
    todo!()
}

pub fn update_comment(
    pool: &Pool,
    changeset: &UpdateComment
) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn delete_comment(
    pool: &Pool,
    comment_id: &CommentID
) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn insert_new_comment(
    pool: &Pool,
    new_comment: &NewComment
) -> Result<Comment, anyhow::Error> {
    todo!()
}

pub struct CommentTreeElement {
    it: Comment,
    children: Vec<CommentTreeElement>
}

pub fn get_comment_tree_for_blog_post(
    pool: &Pool,
    post_id: &BlogPostID,
    page: &Page
) -> Result<Vec<CommentTreeElement>, anyhow::Error> {
    todo!()
}
