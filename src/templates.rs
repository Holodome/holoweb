use askama::Template;
use crate::models;

pub fn render<T>(template: T) -> String
where T: Template
{
    template.render().unwrap()
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "blog_posts.html")]
pub struct PostsTemplate {
    pub posts: Vec<models::BlogPost>
}

#[derive(Template)]
#[template(path = "blog_post.html")]
pub struct PostTemplate {
    pub post: models::BlogPost
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate;