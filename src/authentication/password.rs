use crate::authentication::password::AuthError::InvalidCredentials;
use crate::startup::Pool;
use actix_web::web;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use secrecy::{ExposeSecret, Secret};
use sha3::Digest;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &Pool,
) -> Result<String, AuthError> {
    let password_hash = sha3::Sha3_256::digest(credentials.password.expose_secret().as_bytes());
    let password_hash = format!("{:x}", password_hash);

    if let Some((stored_user_id, stored_password)) =
        get_stored_credentials(&credentials.username, pool).await?
    {
        if stored_password.expose_secret().eq(&password_hash) {
            Ok(stored_user_id)
        } else {
            Err(InvalidCredentials(anyhow::anyhow!("Invalid password")))
        }
    } else {
        Err(InvalidCredentials(anyhow::anyhow!("Invalid username")))
    }
}

#[derive(serde::Deserialize, Queryable)]
struct StoredCredentials {
    user_id: String,
    password: String,
}

async fn get_stored_credentials(
    username: &str,
    pool: &Pool,
) -> Result<Option<(String, Secret<String>)>, anyhow::Error> {
    use crate::diesel::ExpressionMethods;
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    Ok(users
        .filter(name.eq(username))
        .select((id, password))
        .first::<StoredCredentials>(&conn)
        .optional()?
        .map(|c| (c.user_id, Secret::new(c.password))))
}
