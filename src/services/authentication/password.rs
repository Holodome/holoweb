use crate::domain::{Credentials, UserName};
use crate::services::get_stored_credentials;
use crate::startup::Pool;
use secrecy::{ExposeSecret};
use sha3::Digest;

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
    let password_hash =
        sha3::Sha3_256::digest(credentials.password.as_ref().expose_secret().as_bytes());
    let password_hash = format!("{:x}", password_hash);

    if let Some(credentials) = get_stored_credentials(credentials.name, pool).await? {
        if credentials
            .password
            .as_ref()
            .expose_secret()
            .eq(&password_hash)
        {
            Ok(credentials.name.clone())
        } else {
            Err(AuthError::InvalidCredentials(anyhow::anyhow!(
                "Invalid password"
            )))
        }
    } else {
        Err(AuthError::InvalidCredentials(anyhow::anyhow!(
            "Invalid username"
        )))
    }
}
