use crate::domain::{Credentials, HashedUserPassword, UserName};
use crate::services::get_stored_credentials;
use crate::startup::Pool;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &Pool,
) -> Result<UserName, AuthError> {
    if let Some(stored) = get_stored_credentials(credentials.name, pool).await? {
        let hashed_password = HashedUserPassword::parse(&credentials.password, &stored.salt);
        if hashed_password == stored.password {
            Ok(stored.name)
        } else {
            Err(AuthError::InvalidCredentials(anyhow::anyhow!(
                "Passwords don't match"
            )))
        }
    } else {
        Err(AuthError::InvalidCredentials(anyhow::anyhow!(
            "No user with such name found"
        )))
    }
}
