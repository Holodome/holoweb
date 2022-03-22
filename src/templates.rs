use askama::Template;
use crate::domain;

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
    pub posts: Vec<domain::blog_posts::BlogPost>
}

#[derive(Template)]
#[template(path = "blog_post.html")]
pub struct PostTemplate {
    pub post: domain::blog_posts::BlogPost
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate;


#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate;