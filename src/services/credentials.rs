use crate::domain::{Credentials, UserName};
use crate::startup::Pool;


use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
pub async fn get_stored_credentials(
    username: UserName,
    pool: &Pool,
) -> Result<Option<Credentials>, anyhow::Error> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    Ok(users
        .filter(name.eq(username.as_ref().as_str()))
        .select((name, password))
        .first::<Credentials>(&conn)
        .optional()?)
}
