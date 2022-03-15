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

pub fn get_post_by_id(
    db: web::Data<Pool>,
    post_id: Uuid
) -> Result<Option<models::Post>, DbError> {
    let conn = db.get().unwrap();
    let post = posts
        .filter(id.eq(post_id.to_string()))
        .first::<models::Post>(&conn)
        .optional()?;
    log::info!("Found post by id '{:?}': {:?}", post_id, post);
    Ok(post)
}

pub fn get_post_by_name(
    db: web::Data<Pool>,
    post_name: String
) -> Result<Option<models::Post>, DbError> {
    let conn = db.get().unwrap();
    let post = posts
        .filter(name.eq(&post_name))
        .first::<models::Post>(&conn)
        .optional()?;
    log::info!("Found post by name '{:?}': {:?}", &post_name, post);
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
    log::info!("Inserted new post: {:?}", new_post);
    Ok(new_post)
}

pub fn get_all_posts(
    db: web::Data<Pool>
) -> Result<Vec<models::Post>, DbError> {
    let conn = db.get().unwrap();
    let items = posts.load::<models::Post>(&conn)?;
    log::info!("Queried all posts: n={:?}", items.len());
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
    log::info!("Deleted post with id='{:?}'", post_id);
    Ok(())
}