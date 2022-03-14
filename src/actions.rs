use actix_web::web;
use diesel::{insert_into, OptionalExtension};
use uuid::Uuid;
use crate::diesel::ExpressionMethods;
use crate::models::{NewPost, Post};
use crate::Pool;
use crate::schema::posts::dsl::*;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_post_by_id(
    db: web::Data<Pool>,
    post_id: Uuid
) -> Result<Option<Post>, DbError> {
    let conn = db.get().unwrap();
    let post = posts
        .filter(id.eq(post_id.to_string()))
        .first::<Post>(&conn)
        .optional()?;
    Ok(post)
}

pub fn add_new_post(
    db: web::Data<Pool>,
    post: web::Json<NewPost>
) -> Result<Post, DbError> {
    let conn = db.get().unwrap();
    let new_post = Post {
        id: Uuid::new_v4().to_string(),
        name: post.name.clone(),
        contents: post.contents.clone()
    };
    insert_into(posts).values(&new_post).execute(&conn)?;
    Ok(new_post)
}

pub fn get_all_posts(
    db: web::Data<Pool>
) -> Result<Vec<Post>, DbError> {
    let conn = db.get().unwrap();
    let items = posts.load::<Post>(&conn)?;
    Ok(items)
}