use diesel::{insert_into, OptionalExtension, SqliteConnection};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use uuid::Uuid;
use crate::diesel::ExpressionMethods;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::domain;
use crate::domain::users::User;

type DbError = Box<dyn std::error::Error + Send + Sync>;
type Result<T = ()> = std::result::Result<T, DbError>;
type Conn = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_user_by_id(conn: &Conn, user_id: &str)
    -> Result<Option<domain::users::User>> {
    use crate::schema::users::dsl::*;

    Ok(users
        .filter(id.eq(user_id))
        .first::<domain::users::User>(conn)
        .optional()?)
}

pub fn get_user_by_name(conn: &Conn, user_name: &str)
    -> Result<Option<domain::users::User>> {
    use crate::schema::users::dsl::*;

    Ok(users
        .filter(name.eq(user_name))
        .first::<domain::users::User>(conn)
        .optional()?)
}

pub fn get_user_by_email(conn: &Conn, user_email: &str)
    -> Result<Option<domain::users::User>> {
    use crate::schema::users::dsl::*;

    Ok(users
        .filter(email.eq(user_email))
        .first::<domain::users::User>(conn)
        .optional()?)
}

pub fn get_all_users(conn: &Conn)
    -> Result<Vec<domain::users::User>> {
    use crate::schema::users::dsl::*;

    Ok(users
        .load::<domain::users::User>(conn)?
    )
}

pub fn add_user(conn: &Conn, item: domain::users::NewUser)
    -> Result<User> {
    use crate::schema::users::dsl::*;

    let user = domain::users::User {
        id: Uuid::new_v4().to_string(),
        name: item.name.to_string(),
        email: item.email.to_string(),
        password: item.password.to_string(),
        created_at: chrono::Local::now().to_string(),
        role: item.role.to_string(),
        is_banned: false
    };

    insert_into(users).values(&user).execute(conn)?;
    Ok(user)
}

pub fn update_user(conn: &Conn, item: domain::users::UpdateUser)
    -> Result<Option<User>> {
    use crate::schema::users::dsl::*;
    diesel::update(
        users.filter(id.eq(item.id))
    ).set(&item).execute(conn)?;

    get_user_by_id(conn, &item.id)
}

pub fn get_blog_post_by_id(conn: &Conn, post_id: &str)
    -> Result<Option<domain::blog_posts::BlogPost>> {
    use crate::schema::blog_posts::dsl::*;

    Ok(blog_posts
        .filter(id.eq(post_id))
        .first::<domain::blog_posts::BlogPost>(conn)
        .optional()?
    )
}

pub fn get_blog_post_by_title(conn: &Conn, post_title: &str)
    -> Result<Option<domain::blog_posts::BlogPost>> {
    use crate::schema::blog_posts::dsl::*;

    Ok(blog_posts
        .filter(title.eq(post_title))
        .first::<domain::blog_posts::BlogPost>(conn)
        .optional()?
    )
}

pub fn add_blog_post(conn: &Conn, item: domain::blog_posts::NewBlogPost)
    -> Result<domain::blog_posts::BlogPost> {
    use crate::schema::blog_posts::dsl::*;

    let blog_post = domain::blog_posts::BlogPost {
        id: Uuid::new_v4().to_string(),
        title: item.title.to_string(),
        brief: item.brief.unwrap_or(&"").to_string(),
        contents: item.contents.to_string(),
        author_id: item.author_id.to_string()
    };

    insert_into(blog_posts).values(&blog_post).execute(conn)?;
    Ok(blog_post)
}

pub fn update_blog_post(conn: &Conn, item: domain::blog_posts::UpdateBlogPost)
    -> Result<Option<domain::blog_posts::BlogPost>> {
    use crate::schema::blog_posts::dsl::*;

    diesel::update(blog_posts.filter(id.eq(item.id))
    ).set(&item).execute(conn)?;

    get_blog_post_by_id(conn, &item.id)
}

pub fn get_comments_by_post_id(conn: &Conn, required_post_id: &str)
    -> Result<Vec<domain::comments::Comment>> {
    use crate::schema::comments::dsl::*;

    Ok(
        comments
            .filter(post_id.eq(required_post_id))
            .load::<domain::comments::Comment>(conn)?
    )
}