use crate::domain::blog_posts::{
    BlogPost, BlogPostID, BlogPostVisibility, NewBlogPost, UpdateBlogPost,
};
use crate::domain::users::UserID;
use crate::middleware::{Messages, Session};
use crate::routes::error_handlers::ErrorPageTemplate;
use crate::routes::internal::comments::render_regular_comments;
use crate::services::{
    get_all_blog_posts, get_blog_post_by_id, get_comment_views_for_blog_post, insert_new_blog_post,
    update_blog_post,
};
use crate::utils::{e500, redirect_with_error, render_template, see_other};
use crate::Pool;
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use askama::Template;
use secrecy::{ExposeSecret, Secret};

const EDIT_BLOG_POST_CACHE: &str = "edit_blog_post_form";

#[derive(Template)]
#[template(path = "blog_posts.html")]
struct BlogPostsTemplate {
    messages: Messages,
    blog_posts: Vec<BlogPost>,
}

#[tracing::instrument("All blog posts", skip(pool, messages))]
pub async fn all_blog_posts(
    pool: web::Data<Pool>,
    messages: IncomingFlashMessages,
) -> actix_web::Result<HttpResponse> {
    let blog_posts = get_all_blog_posts(&pool).map_err(e500)?;

    render_template(BlogPostsTemplate {
        messages: messages.into(),
        blog_posts,
    })
}

#[derive(Template)]
#[template(path = "blog_post.html", escape = "none")]
struct BlogPostTemplate<'a> {
    messages: Messages,
    blog_post: BlogPost,
    rendered_comments: String,
    csrf_token: &'a str,
}

#[tracing::instrument("Blog post", skip(pool, messages, session))]
pub async fn blog_post(
    pool: web::Data<Pool>,
    params: web::Path<BlogPostID>,
    messages: IncomingFlashMessages,
    current_user_id: Option<UserID>,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let blog_post_id = params.into_inner();
    let blog_post = get_blog_post_by_id(&pool, &blog_post_id)
        .map_err(e500)?
        .ok_or_else(|| actix_web::error::ErrorNotFound("No blog post with such id"))?;

    if blog_post.visibility == BlogPostVisibility::Authenticated && current_user_id.is_none() {
        return render_template(ErrorPageTemplate {
            error_title: "Insufficient permissions",
            error_message: "You have to be authenticated to view this blog post",
            messages: messages.into(),
        });
    }

    let comments = get_comment_views_for_blog_post(&pool, &blog_post_id).map_err(e500)?;
    let rendered_comments =
        render_regular_comments(comments, current_user_id.as_ref()).map_err(e500)?;

    render_template(BlogPostTemplate {
        messages: messages.into(),
        blog_post,
        rendered_comments,
        csrf_token: session.get_csrf_token().map_err(e500)?.expose_secret(),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BlogPostDisplay {
    title: String,
    brief: String,
    contents: String,
}

impl Default for BlogPostDisplay {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            brief: "".to_string(),
            contents: "".to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "edit_blog_post.html")]
struct EditBlogPostTemplate<'a> {
    messages: Messages,
    blog_post: BlogPostDisplay,
    action: &'a str,
    csrf_token: &'a str,
}

#[tracing::instrument("Edit blog post form", skip(pool, messages, session))]
pub async fn edit_blog_post_form(
    pool: web::Data<Pool>,
    params: web::Path<BlogPostID>,
    messages: IncomingFlashMessages,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let blog_post_id = params.into_inner();
    let blog_post = get_blog_post_by_id(&pool, &blog_post_id)
        .map_err(e500)?
        .ok_or_else(|| actix_web::error::ErrorNotFound("No blog post with such id"))?;

    render_template(EditBlogPostTemplate {
        messages: messages.into(),
        blog_post: BlogPostDisplay {
            title: blog_post.title,
            brief: blog_post.brief,
            contents: blog_post.contents,
        },
        action: format!("/blog_posts/{}/edit", blog_post_id.as_ref()).as_str(),
        csrf_token: session.get_csrf_token().map_err(e500)?.expose_secret(),
    })
}

#[derive(serde::Deserialize)]
pub struct EditBlogPostForm {
    title: String,
    brief: String,
    contents: String,
    visible_to_all: Option<String>,
    csrf_token: Secret<String>,
}

#[tracing::instrument("Edit blog post form", skip(pool, form, session))]
pub async fn edit_blog_post(
    pool: web::Data<Pool>,
    form: web::Form<EditBlogPostForm>,
    blog_post_id: web::Path<BlogPostID>,
    session: Session,
) -> Result<HttpResponse, InternalError<anyhow::Error>> {
    let blog_post_id = blog_post_id.into_inner();

    let redirect = |e| {
        redirect_with_error(
            format!("/blog_posts/{}/edit", blog_post_id.as_ref()).as_str(),
            e,
        )
    };

    if form.csrf_token.expose_secret()
        != session.get_csrf_token().map_err(redirect)?.expose_secret()
    {
        return Err(redirect(anyhow::anyhow!("Invalid CSRF token")));
    }

    let changeset = UpdateBlogPost {
        id: &blog_post_id,
        title: Some(&form.title),
        brief: Some(&form.brief),
        contents: Some(&form.contents),
    };
    update_blog_post(&pool, &changeset).map_err(|e| redirect(e.into()))?;
    Ok(see_other(
        format!("/blog_posts/{}/view", blog_post_id.as_ref()).as_str(),
    ))
}

#[tracing::instrument("Create blog post form", skip(messages, session))]
pub async fn create_blog_post_form(
    messages: IncomingFlashMessages,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let blog_post = session
        .pop_form_data::<BlogPostDisplay>(EDIT_BLOG_POST_CACHE)
        .map_err(e500)?
        .unwrap_or_default();
    render_template(EditBlogPostTemplate {
        messages: messages.into(),
        blog_post,
        action: "/blog_posts/create",
        csrf_token: session.get_csrf_token().map_err(e500)?.expose_secret(),
    })
}

#[tracing::instrument("Create blog post", skip(form, pool, session))]
pub async fn create_blog_post(
    // NOTE: Duplicate usage here
    form: web::Form<EditBlogPostForm>,
    pool: web::Data<Pool>,
    user_id: UserID,
    session: Session,
) -> Result<HttpResponse, InternalError<anyhow::Error>> {
    let create_blog_post_redirect = |e| {
        let e = if let Err(new_e) = session.insert_form_data(
            EDIT_BLOG_POST_CACHE,
            BlogPostDisplay {
                title: form.title.clone(),
                brief: form.brief.clone(),
                contents: form.contents.clone(),
            },
        ) {
            anyhow::anyhow!(
                "Failed to execute request: {:?} & failed to cache data: {:?}",
                e,
                new_e
            )
        } else {
            e
        };
        redirect_with_error("/blog_posts/create", e)
    };

    let new_blog_post = NewBlogPost {
        author_id: &user_id,
        title: &form.title,
        brief: &form.brief,
        contents: &form.contents,
        visibility: form
            .visible_to_all
            .as_ref()
            .map(|_| BlogPostVisibility::All)
            .unwrap_or(BlogPostVisibility::Authenticated),
    };
    let blog_post = insert_new_blog_post(&pool, &new_blog_post)
        .map_err(anyhow::Error::new)
        .map_err(create_blog_post_redirect)?;

    Ok(see_other(
        format!("/blog_posts/{}/view", blog_post.id.as_ref()).as_str(),
    ))
}
