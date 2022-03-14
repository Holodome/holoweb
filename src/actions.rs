use actix_web::web;
use diesel::{insert_into, OptionalExtension};
use crate::handlers::InputPost;
use crate::models::{NewPost, Post};
use crate::Pool;
use crate::schema::posts::dsl::*;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_post_by_id(
    db: web::Data<Pool>,
    post_id: i32
) -> Result<Option<Post>, DbError> {
    let conn = db.get().unwrap();
    let post = posts.find(post_id).get_result::<Post>(&conn);
    Ok(post.optional()?)
}

pub fn add_new_post(
    db: web::Data<Pool>,
    post: web::Json<InputPost>
) -> Result<(), DbError> {
    let conn = db.get().unwrap();
    let new_post = NewPost {
        name: &post.name,
        contents: &post.contents
    };
    insert_into(posts).values(&new_post).execute(&conn)?;
    println!("Here\n");
    Ok(())
}

pub fn get_all_posts(
    db: web::Data<Pool>
) -> Result<Vec<Post>, DbError> {
    let conn = db.get().unwrap();
    let items = posts.load::<Post>(&conn)?;
    Ok(items)
}