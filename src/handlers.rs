use actix_web::{web};
use crate::Pool;
use crate::models::Post;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use crate::schema::posts::dsl::*;


pub fn find_post_by_id(
    db: web::Data<Pool>,
    post_id: i32
) -> Result<Post, diesel::result::Error> {
    let conn = db.get().unwrap();
    let post = posts.find(post_id).get_result::<Post>(&conn);
    post
}
