use crate::startup::Pool;
use secrecy::{ExposeSecret, Secret};
use sha3::Digest;
use crate::services::get_stored_credentials;

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

    if let Some(credentials) =
        get_stored_credentials(&credentials.username, pool).await?
    {
        if credentials.user_password.as_ref().expose_secret().eq(&password_hash) {
            Ok(credentials.user_name.as_ref().clone())
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
