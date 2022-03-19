use diesel::{insert_into, OptionalExtension, SqliteConnection};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::diesel::ExpressionMethods;
use crate::models;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

type DbError = Box<dyn std::error::Error + Send + Sync>;
type Result<T = ()> = std::result::Result<T, DbError>;
type Conn = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_user_by_id(conn: &Conn, user_id: &str) -> Result<Option<models::User>> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id))
        .first::<models::User>(conn)
        .optional()?;
    Ok(user)
}

pub fn get_user_by_name(conn: &Conn, user_name: &str) -> Result<Option<models::User>> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(name.eq(user_name))
        .first::<models::User>(conn)
        .optional()?;
    Ok(user)
}

pub fn get_user_by_email(conn: &Conn, user_email: &str) -> Result<Option<models::User>> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(email.eq(user_email))
        .first::<models::User>(conn)
        .optional()?;
    Ok(user)
}
