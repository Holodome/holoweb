use crate::domain::blog_posts::BlogPostID;
use crate::domain::comments::{Comment, CommentID, NewComment, UpdateComment};
use crate::domain::time::DateTime;
use crate::domain::users::UserID;
use crate::schema::comments::dsl::*;
use crate::Pool;
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
) -> Result<Vec<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(author_id.eq(post_author_id))
        .load::<Comment>(&conn)?)
}

pub fn get_comments_for_blog_post(
    pool: &Pool,
    blog_post_id: &BlogPostID,
) -> Result<Vec<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(post_id.eq(blog_post_id))
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
    let time = DateTime::now();
    let comment = Comment {
        id: CommentID::generate_random(),
        author_id: new_comment.author_id.clone(),
        post_id: new_comment.post_id.clone(),
        reply_to_id: new_comment.parent_id.cloned(),
        contents: new_comment.contents.to_string(),
        created_at: time.clone(),
        updated_at: time,
        is_deleted: false,
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
        .filter(reply_to_id.is_null())
        .load::<Comment>(&conn)?)
}

pub fn get_child_comments(
    pool: &Pool,
    comment_id: &CommentID,
) -> Result<Vec<Comment>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(comments
        .filter(reply_to_id.eq(comment_id))
        .load::<Comment>(&conn)?)
}
