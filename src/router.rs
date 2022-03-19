// use actix_web::{get, Result, HttpResponse, web};
// use crate::{services, Pool, templates};
//
//
// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(index_page)
//         .service(blog_posts_page)
//         .service(blog_post_page)
//     ;
// }
//
// #[get("/")]
// async fn index_page() -> Result<HttpResponse> {
//     let s = templates::render(templates::IndexTemplate);
//     Ok(HttpResponse::Ok().content_type("text/html").body(s))
// }
//
// #[get("/blog_posts")]
// async fn blog_posts_page(pool: web::Data<Pool>) -> Result<HttpResponse> {
//     let s = templates::render(templates::PostsTemplate {
//         posts: services::get_all_posts(pool)
//             .unwrap_or(Vec::default())
//     });
//     Ok(HttpResponse::Ok().content_type("text/html").body(s))
// }
//
// #[get("/blog_post/{name}")]
// async fn blog_post_page(pool: web::Data<Pool>, name: web::Path<String>) -> Result<HttpResponse> {
//     let post = services::get_post_by_name(pool, name.into_inner());
//     let resp = match post {
//             Ok(Some(post)) => {
//                 let s = templates::render(templates::PostTemplate {
//                     post
//                 });
//                 HttpResponse::Ok().content_type("text/html").body(s)
//             },
//             _ => HttpResponse::NotFound().finish()
//     };
//     Ok(resp)
// }
