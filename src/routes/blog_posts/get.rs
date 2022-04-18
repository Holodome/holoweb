use crate::domain::blog_posts::{BlogPost, BlogPostID};
use crate::domain::users::UserID;
use crate::services::{get_all_blog_posts, get_blog_post_by_id, get_comments_for_blog_post};
use crate::utils::{e500, render_template};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::domain::comments::{Comment, CommentID};
use crate::Pool;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use diesel::insert_into;
use serde::de::Unexpected::Str;

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

fn render_comments(comments: Vec<Comment>) -> Result<String, anyhow::Error> {
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

    let mut visited = HashSet::<&str>::new();
    let mut rendered = HashMap::<&str, String>::new();
    let mut stack = VecDeque::from(orphans);
    while !stack.is_empty() {
        let current = stack.pop_front().unwrap();
        let current_id = current.id.as_ref().as_str();
        let children = children.entry(current_id).or_default();

        if visited.contains(current_id) {
            // TODO: Normal sorting
            children.sort_by(|a, b| a.contents.cmp(&b.contents));
            let rendered_children = children
                .iter()
                .map(|c| rendered.remove(c.id.as_ref().as_str()).unwrap())
                .collect();

            let s = CommentRender {
                author: "TODO",
                date: "TODO",
                contents: current.contents.as_str(),
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

    Ok(rendered.into_values().collect::<Vec<_>>().join(""))
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

    render_template(BlogPostTemplate {
        messages,
        blog_post,
        rendered_comments: "".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove_spaces(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    #[test]
    fn render_comment_works() {
        let comments = vec![Comment {
            id: CommentID::generate_random(),
            contents: "hello world".to_string(),
            author_id: UserID::generate_random(),
            post_id: BlogPostID::generate_random(),
            reply_to_id: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
            is_deleted: false,
        }];
        let rendered = render_comments(comments).unwrap();
        let expected = r#"
<div class="comment">
  <div class="content">
    <a class="author">TODO</a>
    <div class="metadata">
      <span class="date">TODO</span>
    </div>
    <div class="text">
      <p>hello world</p>
    </div>
    <div class="actions">
      <a class="reply">Reply</a>
    </div>
  </div>
  <div class="comments">
  </div>
</div>"#
            .to_string();
        let expected_without_spaces = remove_spaces(&expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }

    #[test]
    fn render_reply_works() {
        let id0 = CommentID::generate_random();
        let comments = vec![
            Comment {
                id: CommentID::generate_random(),
                contents: "world".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: Some(id0.clone()),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: id0,
                contents: "hello".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: None,
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
        ];
        let rendered = render_comments(comments).unwrap();
        let expected = r#"
<div class="comment">
  <div class="content">
    <a class="author">TODO</a>
    <div class="metadata">
      <span class="date">TODO</span>
    </div>
    <div class="text">
      <p>hello</p>
    </div>
    <div class="actions">
      <a class="reply">Reply</a>
    </div>
  </div>
  <div class="comments">
    <div class="comment">
      <div class="content">
        <a class="author">TODO</a>
        <div class="metadata">
          <span class="date">TODO</span>
        </div>
        <div class="text">
          <p>world</p>
        </div>
        <div class="actions">
          <a class="reply">Reply</a>
        </div>
      </div>
      <div class="comments">
      </div>
    </div>
  </div>
</div>"#;
        let expected_without_spaces = remove_spaces(expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }

    #[test]
    fn render_multiple_replies_works() {
        let id0 = CommentID::generate_random();
        let comments = vec![
            Comment {
                id: CommentID::generate_random(),
                contents: "2".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: Some(id0.clone()),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: CommentID::generate_random(),
                contents: "3".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: Some(id0.clone()),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: id0,
                contents: "1".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: None,
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
        ];
        let rendered = render_comments(comments).unwrap();
        let expected = r#"
<div class="comment">
  <div class="content">
    <a class="author">TODO</a>
    <div class="metadata">
      <span class="date">TODO</span>
    </div>
    <div class="text">
      <p>1</p>
    </div>
    <div class="actions">
      <a class="reply">Reply</a>
    </div>
  </div>
  <div class="comments">
    <div class="comment">
      <div class="content">
        <a class="author">TODO</a>
        <div class="metadata">
          <span class="date">TODO</span>
        </div>
        <div class="text">
          <p>2</p>
        </div>
        <div class="actions">
          <a class="reply">Reply</a>
        </div>
      </div>
      <div class="comments">
      </div>
    </div>
    <div class="comment">
      <div class="content">
        <a class="author">TODO</a>
        <div class="metadata">
          <span class="date">TODO</span>
        </div>
        <div class="text">
          <p>3</p>
        </div>
        <div class="actions">
          <a class="reply">Reply</a>
        </div>
      </div>
      <div class="comments">
      </div>
    </div>
  </div>
</div>"#;
        let expected_without_spaces = remove_spaces(expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }

    #[test]
    fn render_multiple_toplevel_comments() {
        let comments = vec![
            Comment {
                id: CommentID::generate_random(),
                contents: "2".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: None,
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: CommentID::generate_random(),
                contents: "3".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: None,
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
        ];
        let rendered = render_comments(comments).unwrap();
        let expected = r#"
<div class="comment">
  <div class="content">
    <a class="author">TODO</a>
    <div class="metadata">
      <span class="date">TODO</span>
    </div>
    <div class="text">
      <p>2</p>
    </div>
    <div class="actions">
      <a class="reply">Reply</a>
    </div>
  </div>
  <div class="comments">
  </div>
</div>
<div class="comment">
  <div class="content">
    <a class="author">TODO</a>
    <div class="metadata">
      <span class="date">TODO</span>
    </div>
    <div class="text">
      <p>3</p>
    </div>
    <div class="actions">
      <a class="reply">Reply</a>
    </div>
  </div>
  <div class="comments">
  </div>
</div>
"#;
        let expected_without_spaces = remove_spaces(expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }

    #[test]
    fn render_multiple_levels_of_nesting_and_multiple_children_works() {
        let id0 = CommentID::generate_random();
        let id1 = CommentID::generate_random();
        let id2 = CommentID::generate_random();
        let comments = vec![
            Comment {
                id: id0.clone(),
                contents: "1".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: None,
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: CommentID::generate_random(),
                contents: "2".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: Some(id0.clone()),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: id1.clone(),
                contents: "3".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: Some(id0.clone()),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: id2.clone(),
                contents: "4".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: Some(id1.clone()),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: CommentID::generate_random(),
                contents: "5".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: Some(id2.clone()),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
            Comment {
                id: CommentID::generate_random(),
                contents: "6".to_string(),
                author_id: UserID::generate_random(),
                post_id: BlogPostID::generate_random(),
                reply_to_id: Some(id2.clone()),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                is_deleted: false,
            },
        ];
        let rendered = render_comments(comments).unwrap();
        let expected = r#"
        <div class="comment">
  <div class="content">
    <a class="author">TODO</a>
    <div class="metadata">
      <span class="date">TODO</span>
    </div>
    <div class="text">
      <p>1</p>
    </div>
    <div class="actions">
      <a class="reply">Reply</a>
    </div>
  </div>
  <div class="comments">
    <div class="comment">
      <div class="content">
        <a class="author">TODO</a>
        <div class="metadata">
          <span class="date">TODO</span>
        </div>
        <div class="text">
          <p>2</p>
        </div>
        <div class="actions">
          <a class="reply">Reply</a>
        </div>
      </div>
      <div class="comments">
      </div>
    </div>
    <div class="comment">
      <div class="content">
        <a class="author">TODO</a>
        <div class="metadata">
          <span class="date">TODO</span>
        </div>
        <div class="text">
          <p>3</p>
        </div>
        <div class="actions">
          <a class="reply">Reply</a>
        </div>
      </div>
      <div class="comments">
        <div class="comment">
          <div class="content">
            <a class="author">TODO</a>
            <div class="metadata">
              <span class="date">TODO</span>
            </div>
            <div class="text">
              <p>4</p>
            </div>
            <div class="actions">
              <a class="reply">Reply</a>
            </div>
          </div>
          <div class="comments">
            <div class="comment">
              <div class="content">
                <a class="author">TODO</a>
                <div class="metadata">
                  <span class="date">TODO</span>
                </div>
                <div class="text">
                  <p>5</p>
                </div>
                <div class="actions">
                  <a class="reply">Reply</a>
                </div>
              </div>
              <div class="comments">
              </div>
            </div>
            <div class="comment">
              <div class="content">
                <a class="author">TODO</a>
                <div class="metadata">
                  <span class="date">TODO</span>
                </div>
                <div class="text">
                  <p>6</p>
                </div>
                <div class="actions">
                  <a class="reply">Reply</a>
                </div>
              </div>
              <div class="comments">
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
"#;
        let expected_without_spaces = remove_spaces(expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }
}
