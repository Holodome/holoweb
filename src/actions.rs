use actix_web::web;
use diesel::{insert_into, OptionalExtension};
use uuid::Uuid;
use crate::diesel::ExpressionMethods;
use crate::Pool;
use crate::schema::posts::dsl::*;
use crate::models;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_post_by_id(
    db: web::Data<Pool>,
    post_id: Uuid
) -> Result<Option<models::Post>, DbError> {
    let conn = db.get().unwrap();
    let post = posts
        .filter(id.eq(post_id.to_string()))
        .first::<models::Post>(&conn)
        .optional()?;
    Ok(post)
}

pub fn add_new_post(
    db: web::Data<Pool>,
    post: web::Json<models::NewPost>
) -> Result<models::Post, DbError> {
    let conn = db.get().unwrap();
    let new_post = models::Post {
        id: Uuid::new_v4().to_string(),
        name: post.name.clone(),
        contents: post.contents.clone()
    };
    insert_into(posts).values(&new_post).execute(&conn)?;
    Ok(new_post)
}

pub fn get_all_posts(
    db: web::Data<Pool>
) -> Result<Vec<models::Post>, DbError> {
    let conn = db.get().unwrap();
    let items = posts.load::<models::Post>(&conn)?;
    Ok(items)
}

pub fn delete_post_by_id(
    db: web::Data<Pool>,
    post_id: Uuid
) -> Result<(), DbError> {
    let conn = db.get().unwrap();
    diesel::delete(
        posts
            .filter(id.eq(post_id.to_string()))
    ).execute(&conn)?;
    Ok(())
}