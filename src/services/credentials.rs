use crate::startup::Pool;

use crate::domain::users::{StoredCredentials, UserName};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
pub fn get_stored_credentials(
    username: UserName,
    pool: &Pool,
) -> Result<Option<StoredCredentials>, anyhow::Error> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    Ok(users
        .filter(name.eq(username.as_ref().as_str()))
        .select((name, password, password_salt, id))
        .first::<StoredCredentials>(&conn)
        .optional()?)
}
