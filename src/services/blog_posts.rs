use crate::domain::blog_posts::{BlogPost, BlogPostID, NewBlogPost, UpdateBlogPost};
use crate::domain::users::UserID;
use crate::schema::blog_posts::dsl::*;
use crate::services::{get_current_time_str, Page};
use crate::Pool;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{insert_into, update, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use std::fmt::Formatter;

#[derive(thiserror::Error)]
pub enum BlogPostError {
    #[error("Title is already taken")]
    TakenTitle,
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for BlogPostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::utils::error_chain_fmt;

        error_chain_fmt(self, f)
    }
}

pub fn get_blog_post_by_id(
    pool: &Pool,
    blog_post_id: &BlogPostID,
) -> Result<Option<BlogPost>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(blog_posts
        .filter(id.eq(blog_post_id))
        .first::<BlogPost>(&conn)
        .optional()?)
}

pub fn get_blog_post_by_title(
    pool: &Pool,
    blog_post_title: &str,
) -> Result<Option<BlogPost>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(blog_posts
        .filter(title.eq(blog_post_title))
        .first::<BlogPost>(&conn)
        .optional()?)
}

pub fn insert_new_blog_post(
    pool: &Pool,
    new_blog_post: &NewBlogPost,
) -> Result<BlogPost, BlogPostError> {
    let conn = pool
        .get()
        .map_err(|e| BlogPostError::UnexpectedError(e.into()))?;
    let blog_post = BlogPost {
        id: BlogPostID::generate_random(),

        title: new_blog_post.title.to_string(),
        brief: new_blog_post.brief.to_string(),
        contents: new_blog_post.contents.to_string(),
        author_id: new_blog_post.author_id.clone(),
        created_at: get_current_time_str(),
        visibility: "all".to_string(),
    };
    insert_into(blog_posts)
        .values(&blog_post)
        .execute(&conn)
        .map_err(get_blog_post_error_error_from_database_error)?;
    Ok(blog_post)
}

pub fn update_blog_post(pool: &Pool, changeset: &UpdateBlogPost) -> Result<(), BlogPostError> {
    let conn = pool
        .get()
        .map_err(|e| BlogPostError::UnexpectedError(e.into()))?;
    update(blog_posts.filter(id.eq(&changeset.id)))
        .set(changeset)
        .execute(&conn)
        .map_err(get_blog_post_error_error_from_database_error)?;
    Ok(())
}

pub fn get_blog_posts_of_author(
    pool: &Pool,
    author: UserID,
    page: &Page,
) -> Result<Vec<BlogPost>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(blog_posts
        .filter(author_id.eq(author))
        .offset((page.number * page.size) as i64)
        .limit(page.size as i64)
        .load::<BlogPost>(&conn)?)
}

pub fn get_all_blog_posts(pool: &Pool, page: &Page) -> Result<Vec<BlogPost>, anyhow::Error> {
    let conn = pool.get()?;
    Ok(blog_posts
        .offset((page.number * page.size) as i64)
        .limit(page.size as i64)
        .load::<BlogPost>(&conn)?)
}

fn get_blog_post_error_error_from_database_error(e: Error) -> BlogPostError {
    match e {
        Error::DatabaseError(DatabaseErrorKind::UniqueViolation, ref data) => {
            let msg = data.message();
            if msg.contains("title") {
                BlogPostError::TakenTitle
            } else {
                BlogPostError::UnexpectedError(e.into())
            }
        }
        _ => BlogPostError::UnexpectedError(e.into()),
    }
}
