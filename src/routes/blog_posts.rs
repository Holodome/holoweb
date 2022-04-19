use crate::domain::blog_posts::{BlogPost, BlogPostID};
use crate::services::{get_all_blog_posts, get_blog_post_by_id, get_comments_for_blog_post};
use crate::utils::{e500, render_template};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::domain::comments::Comment;
use crate::Pool;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;

#[derive(Template)]
#[template(path = "blog_posts.html")]
struct BlogPostsTemplate {
    messages: IncomingFlashMessages,
    blog_posts: Vec<BlogPost>,
}

#[tracing::instrument("All blog posts", skip(pool, messages))]
pub async fn all_blog_posts(
    pool: web::Data<Pool>,
    messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let blog_posts = get_all_blog_posts(&pool).map_err(e500)?;

    render_template(BlogPostsTemplate {
        messages,
        blog_posts,
    })
}

#[derive(Template)]
#[template(path = "comment.html", escape = "none")]
struct CommentRender<'a> {
    author: &'a str,
    date: &'a str,
    contents: &'a str,
    rendered_children: Vec<String>,
}

fn render_comments<F>(comments: Vec<Comment>, mut comparator: F) -> Result<String, anyhow::Error>
where
    F: FnMut(&&Comment, &&Comment) -> core::cmp::Ordering,
{
    let mut children = HashMap::<&str, Vec<&Comment>>::new();
    let mut orphans = Vec::new();
    for comment in comments.iter() {
        if let Some(reply_to_id) = &comment.reply_to_id {
            children
                .entry(reply_to_id.as_ref().as_str())
                .or_default()
                .push(comment);
        } else {
            orphans.push(comment);
        }
    }

    orphans.sort_by(&mut comparator);

    let mut visited = HashSet::<&str>::new();
    let mut rendered = HashMap::<&str, String>::new();
    let mut stack = VecDeque::from(orphans.clone());
    while !stack.is_empty() {
        let current = stack.pop_front().unwrap();
        let current_id = current.id.as_ref().as_str();
        let children = children.entry(current_id).or_default();

        if visited.contains(current_id) {
            children.sort_by(&mut comparator);
            let rendered_children = children
                .iter()
                .map(|c| rendered.remove(c.id.as_ref().as_str()).unwrap())
                .collect();

            // TODO: Handle deleted in render
            let contents = if current.is_deleted {
                "<deleted>"
            } else {
                current.contents.as_str()
            };
            // TODO: Author
            // TODO: Date
            let s = CommentRender {
                author: "TODO",
                date: "TODO",
                contents,
                rendered_children,
            }
            .render()?;
            rendered.insert(current_id, s);

            continue;
        } else {
            visited.insert(current_id);
        }

        stack.push_front(current);
        for child in children {
            stack.push_front(child);
        }
    }

    Ok(orphans
        .iter()
        .map(|o| rendered.remove(o.id.as_ref().as_str()).unwrap())
        .collect::<Vec<String>>()
        .join(""))
}

#[derive(Template)]
#[template(path = "blog_post.html")]
struct BlogPostTemplate {
    messages: IncomingFlashMessages,
    blog_post: BlogPost,
    rendered_comments: String,
}

#[tracing::instrument("Blog post", skip(pool, messages))]
pub async fn blog_post(
    pool: web::Data<Pool>,
    params: web::Path<BlogPostID>,
    messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let blog_post_id = params.into_inner();
    let blog_post = get_blog_post_by_id(&pool, &blog_post_id)
        .map_err(e500)?
        .ok_or_else(|| actix_web::error::ErrorNotFound("No blog post with such id"))?;

    let comments = get_comments_for_blog_post(&pool, &blog_post_id).map_err(e500)?;
    let rendered_comments =
        render_comments(comments, |a, b| a.contents.cmp(&b.contents)).map_err(e500)?;

    render_template(BlogPostTemplate {
        messages,
        blog_post,
        rendered_comments,
    })
}

struct BlogPostDisplay<'a> {
    title: &'a str,
    brief: &'a str,
    contents: &'a str,
}

impl<'a> BlogPostDisplay<'a> {
    pub fn new(b: &'a BlogPost) -> Self {
        Self {
            title: b.title.as_str(),
            brief: b.brief.as_str(),
            contents: b.contents.as_str(),
        }
    }
}

impl Default for BlogPostDisplay<'static> {
    fn default() -> Self {
        Self {
            title: "Untitled",
            brief: "",
            contents: "",
        }
    }
}

#[derive(Template)]
#[template(path = "edit_blog_post.html")]
struct EditBlogPostTemplate<'a> {
    messages: IncomingFlashMessages,
    blog_post: BlogPostDisplay<'a>,
}

#[tracing::instrument("Edit blog post", skip(pool, messages))]
pub async fn edit_blog_post_form(
    pool: web::Data<Pool>,
    params: web::Path<BlogPostID>,
    messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let blog_post_id = params.into_inner();
    let blog_post = get_blog_post_by_id(&pool, &blog_post_id).map_err(e500)?;
    let blog_post = blog_post.ok_or_else(|| e500(anyhow::anyhow!("TODO")))?;

    render_template(EditBlogPostTemplate {
        messages,
        blog_post: BlogPostDisplay::new(&blog_post),
    })
}

#[tracing::instrument("Create blog post", skip(messages))]
pub async fn create_blog_post_form(
    messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let blog_post = BlogPostDisplay::default();
    render_template(EditBlogPostTemplate {
        messages,
        blog_post,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::comments::CommentID;
    use crate::domain::users::UserID;

    fn remove_spaces(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    fn test_render_comments(comments: Vec<Comment>) -> Result<String, anyhow::Error> {
        render_comments(comments, |a, b| a.contents.cmp(&b.contents))
    }

    fn generate_comment(
        contents: String,
        id: Option<CommentID>,
        reply_to: Option<CommentID>,
    ) -> Comment {
        Comment {
            id: id.unwrap_or_else(|| CommentID::generate_random()),
            contents,
            author_id: UserID::generate_random(),
            post_id: BlogPostID::generate_random(),
            reply_to_id: reply_to,
            created_at: "".to_string(),
            updated_at: "".to_string(),
            is_deleted: false,
        }
    }

    #[test]
    fn render_comment_works() {
        let comments = vec![generate_comment("hello world".to_string(), None, None)];
        let rendered = test_render_comments(comments).unwrap();
        let expected =
            include_str!("../../tests/data/render_comments_render_comment.html").to_string();

        let expected_without_spaces = remove_spaces(&expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }

    #[test]
    fn render_reply_works() {
        let id0 = CommentID::generate_random();
        let comments = vec![
            generate_comment("hello".to_string(), Some(id0.clone()), None),
            generate_comment("world".to_string(), None, Some(id0.clone())),
        ];
        let rendered = test_render_comments(comments).unwrap();
        let expected =
            include_str!("../../tests/data/render_comments_render_reply.html").to_string();

        let expected_without_spaces = remove_spaces(&expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }

    #[test]
    fn render_multiple_replies_works() {
        let id0 = CommentID::generate_random();
        let comments = vec![
            generate_comment("1".to_string(), Some(id0.clone()), None),
            generate_comment("2".to_string(), None, Some(id0.clone())),
            generate_comment("3".to_string(), None, Some(id0.clone())),
        ];
        let rendered = test_render_comments(comments).unwrap();
        let expected =
            include_str!("../../tests/data/render_comments_render_multiple_replies.html")
                .to_string();

        let expected_without_spaces = remove_spaces(&expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }

    #[test]
    fn render_multiple_toplevel_comments() {
        let comments = vec![
            generate_comment("2".to_string(), None, None),
            generate_comment("3".to_string(), None, None),
        ];
        let rendered = test_render_comments(comments).unwrap();
        let expected =
            include_str!("../../tests/data/render_comments_render_multiple_toplevel_comments.html")
                .to_string();

        let expected_without_spaces = remove_spaces(&expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }

    #[test]
    fn render_multiple_levels_of_nesting_and_multiple_children_works() {
        let id0 = CommentID::generate_random();
        let id1 = CommentID::generate_random();
        let id2 = CommentID::generate_random();
        let comments = vec![
            generate_comment("1".to_string(), Some(id0.clone()), None),
            generate_comment("2".to_string(), None, Some(id0.clone())),
            generate_comment("3".to_string(), Some(id1.clone()), Some(id0.clone())),
            generate_comment("4".to_string(), Some(id2.clone()), Some(id1.clone())),
            generate_comment("5".to_string(), None, Some(id2.clone())),
            generate_comment("6".to_string(), None, Some(id2.clone())),
        ];
        let rendered = test_render_comments(comments).unwrap();
        let expected = include_str!("../../tests/data/render_comments_render_multiple_levels_of_nesting_and_multiple_children.html").to_string();

        let expected_without_spaces = remove_spaces(&expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }
}
