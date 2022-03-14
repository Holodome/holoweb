use actix_web::web;
use diesel::insert_into;
use crate::handlers::InputPost;
use crate::models::{NewPost, Post};
use crate::Pool;
use crate::schema::posts::dsl::*;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

pub fn find_post_by_id(
    db: web::Data<Pool>,
    post_id: i32
) -> Result<Post, diesel::result::Error> {
    let conn = db.get().unwrap();
    let post = posts.find(post_id).get_result::<Post>(&conn);
    post
}

pub fn add_new_post(
    db: web::Data<Pool>,
    post: web::Json<InputPost>
) -> Result<(), diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_post = NewPost {
        name: &post.name,
        contents: &post.contents
    };
    insert_into(posts).values(&new_post).execute(&conn)?;
    Ok(())
}