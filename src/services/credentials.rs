use crate::domain::Credentials;
use crate::startup::Pool;
use secrecy::Secret;

use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, ExpressionMethods};
use crate::services::DbError;

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
pub async fn get_stored_credentials(
    username: &str,
    pool: &Pool,
) -> Result<Option<Credentials>, anyhow::Error> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    Ok(users
        .filter(name.eq(username))
        .select((name, password))
        .first::<Credentials>(&conn)
        .optional()?
    )
}
