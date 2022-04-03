use crate::domain::blog_posts::{BlogPost, BlogPostID, NewBlogPost, UpdateBlogPost};
use crate::schema::blog_posts::dsl::*;
use crate::startup::Pool;
use diesel::{insert_into, update, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

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
    blog_post_title: String,
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
) -> Result<BlogPost, anyhow::Error> {
    let conn = pool.get()?;
    let blog_post = BlogPost {
        id: BlogPostID::generate_random(),

        title: new_blog_post.title.clone(),
        brief: new_blog_post.brief.clone(),
        contents: new_blog_post.contents.clone(),
        author_id: new_blog_post.author_id.clone(),
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            .to_string(),
    };
    insert_into(blog_posts).values(&blog_post).execute(&conn)?;
    Ok(blog_post)
}

pub fn update_blog_post(pool: &Pool, changeset: &UpdateBlogPost) -> Result<(), anyhow::Error> {
    let conn = pool.get()?;
    update(blog_posts.filter(id.eq(&changeset.id)))
        .set(changeset)
        .execute(&conn)?;
    Ok(())
}
