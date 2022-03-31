use crate::domain::{NewUser, User, UserEmail, UserID};
use crate::schema::users::dsl::*;
use crate::services::Connection;
use diesel::{insert_into, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

#[tracing::instrument("Get user by id", skip(conn, user_id))]
pub fn get_user_by_id(conn: &Connection, user_id: &str) -> Result<Option<User>, anyhow::Error> {
    Ok(users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?)
}

pub fn get_user_by_name(conn: &Connection, user_name: &str) -> Result<Option<User>, anyhow::Error> {
    Ok(users
        .filter(name.eq(user_name))
        .first::<User>(conn)
        .optional()?)
}

pub fn insert_new_user(conn: &Connection, new_user: NewUser) -> Result<User, anyhow::Error> {
    let user = User {
        id: UserID::generate_random(),
        name: new_user.name,
        email: UserEmail::parse("hello@email.com".to_string()).expect("Oh no"), // TODO
        password: new_user.password,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            .to_string(),
        is_banned: false,
    };

    insert_into(users).values(&user).execute(conn)?;

    Ok(user)
}
