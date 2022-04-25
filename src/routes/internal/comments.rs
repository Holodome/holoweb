use crate::domain::comments::CommentView;
use crate::domain::time::DateTime;
use crate::domain::users::UserID;
use askama::Template;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Template)]
#[template(path = "comment.html", escape = "none")]
struct CommentTemplate<'a> {
    pub author: &'a str,
    pub date: &'a str,
    pub contents: &'a str,
    pub rendered_children: Vec<String>,
    pub id: &'a str,
    pub is_comment_author: bool,
}

pub struct RenderCommentData<'a> {
    id: &'a str,
    author: &'a str,
    date: &'a str,
    contents: &'a str,
    rendered_children: Vec<String>,
    is_comment_author: bool,
}

pub fn render_regular_comments(
    comments: Vec<CommentView>,
    current_user: Option<&UserID>,
) -> Result<String, anyhow::Error> {
    render_comments(
        comments,
        current_user,
        |a, b| a.contents.cmp(&b.contents),
        render_comment,
    )
}

fn render_comment(data: RenderCommentData) -> Result<String, anyhow::Error> {
    CommentTemplate {
        author: data.author,
        date: data.date,
        contents: data.contents,
        rendered_children: data.rendered_children,
        id: data.id,
        is_comment_author: data.is_comment_author,
    }
    .render()
    .map_err(|e| anyhow::anyhow!("Failed to render comment: {:?}", e))
}

fn render_comments<F, T>(
    comments: Vec<CommentView>,
    current_user_id: Option<&UserID>,
    mut comparator: F,
    mut renderer: T,
) -> Result<String, anyhow::Error>
where
    F: FnMut(&&CommentView, &&CommentView) -> core::cmp::Ordering,
    T: FnMut(RenderCommentData) -> Result<String, anyhow::Error>,
{
    let mut children = HashMap::<&str, Vec<&CommentView>>::new();
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

    let current_time = DateTime::now();

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
                "deleted"
            } else {
                current.contents.as_str()
            };
            let s = renderer(RenderCommentData {
                id: current.id.as_ref(),
                author: current.author_name.as_ref(),
                date: &current.created_at.since(&current_time),
                contents,
                rendered_children,
                is_comment_author: current_user_id
                    .map(|it| &current.author_id == it)
                    .unwrap_or(false),
            })?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::blog_posts::BlogPostID;
    use crate::domain::comments::CommentID;
    use crate::domain::time::DateTime;
    use crate::domain::users::UserName;

    fn remove_spaces(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    #[derive(Template)]
    #[template(
        source = r#"<div class="comment">
  <div class="content">
    <div class="text">
      <p>{{ contents }}</p>
    </div>
    <div class="actions">
      <a class="reply">Reply</a>
    </div>
  </div>
  {% if !rendered_children.is_empty() %}
    <div class="comments">
      {% for child in rendered_children %}
        {{ child }}
      {% endfor %}
    </div>
  {% endif %}
</div>
"#,
        ext = "html",
        escape = "none"
    )]
    struct TestCommentTemplate<'a> {
        pub contents: &'a str,
        pub rendered_children: Vec<String>,
    }

    fn test_render_comment(data: RenderCommentData) -> Result<String, anyhow::Error> {
        TestCommentTemplate {
            contents: data.contents,
            rendered_children: data.rendered_children,
        }
        .render()
        .map_err(|e| anyhow::anyhow!("Failed to render comment: {:?}", e))
    }

    fn test_render_comments(comments: Vec<CommentView>) -> Result<String, anyhow::Error> {
        render_comments(
            comments,
            None,
            |a, b| a.contents.cmp(&b.contents),
            test_render_comment,
        )
    }

    fn generate_comment(
        contents: String,
        id: Option<CommentID>,
        reply_to: Option<CommentID>,
    ) -> CommentView {
        let time = DateTime::now();
        CommentView {
            id: id.unwrap_or_else(|| CommentID::generate_random()),
            contents,
            author_id: UserID::generate_random(),
            author_name: UserName::generate_random(),
            post_id: BlogPostID::generate_random(),
            reply_to_id: reply_to,
            created_at: time.clone(),
            updated_at: time,
            is_deleted: false,
        }
    }

    #[test]
    fn render_comment_works() {
        let comments = vec![generate_comment("hello world".to_string(), None, None)];
        let rendered = test_render_comments(comments).unwrap();
        let expected =
            include_str!("../../../tests/data/render_comments_render_comment.html").to_string();

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
            include_str!("../../../tests/data/render_comments_render_reply.html").to_string();

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
            include_str!("../../../tests/data/render_comments_render_multiple_replies.html")
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
        let expected = include_str!(
            "../../../tests/data/render_comments_render_multiple_toplevel_comments.html"
        )
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
        let expected = include_str!("../../../tests/data/render_comments_render_multiple_levels_of_nesting_and_multiple_children.html").to_string();

        let expected_without_spaces = remove_spaces(&expected);
        let rendered_without_spaces = remove_spaces(&rendered);
        assert_eq!(rendered_without_spaces, expected_without_spaces);
    }
}
