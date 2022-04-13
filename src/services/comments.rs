use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::{Comment, CommentID, NewComment, UpdateComment};
use crate::domain::users::UserID;
use crate::schema::comments::dsl::*;
use crate::services::{get_current_time_str, Page};
use crate::startup::Pool;
use diesel::{insert_into, update, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

pub fn get_comment_by_id(
    pool: &Pool,
    comment_id: &CommentID,
) -> Result<Option<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(id.eq(comment_id))
        .first::<Comment>(&conn)
        .optional()?)
}

pub fn get_comments_of_author(
    pool: &Pool,
    post_author_id: &UserID,
    page: &Page,
) -> Result<Vec<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(author_id.eq(post_author_id))
        .offset((page.number * page.size) as i64)
        .limit(page.size as i64)
        .load::<Comment>(&conn)?)
}

pub fn get_comments_for_blog_post(
    pool: &Pool,
    blog_post_id: &BlogPostID,
    page: &Page,
) -> Result<Vec<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(post_id.eq(blog_post_id))
        .offset((page.number * page.size) as i64)
        .limit(page.size as i64)
        .load::<Comment>(&conn)?)
}

pub fn update_comment(pool: &Pool, changeset: &UpdateComment) -> Result<(), anyhow::Error> {
    let conn = pool.get()?;
    update(comments.filter(id.eq(&changeset.id)))
        .set(changeset)
        .execute(&conn)?;
    Ok(())
}

pub fn insert_new_comment(pool: &Pool, new_comment: &NewComment) -> Result<Comment, anyhow::Error> {
    let conn = pool.get()?;
    let comment = Comment {
        id: CommentID::generate_random(),
        author_id: new_comment.author_id.clone(),
        post_id: new_comment.post_id.clone(),
        parent_id: new_comment.parent_id.cloned(),
        contents: new_comment.contents.to_string(),
        created_at: get_current_time_str(),
        is_deleted: false,
        main_parent_id: None,
        depth: 0,
    };
    insert_into(comments).values(&comment).execute(&conn)?;
    Ok(comment)
}

pub fn get_toplevel_comments_for_blog_post(
    pool: &Pool,
    blog_post_id: &BlogPostID,
) -> Result<Vec<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(post_id.eq(blog_post_id))
        .filter(parent_id.is_null())
        .load::<Comment>(&conn)?)
}

pub fn get_child_comments(
    pool: &Pool,
    comment_id: &CommentID,
) -> Result<Vec<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(parent_id.eq(comment_id))
        .load::<Comment>(&conn)?)
}
