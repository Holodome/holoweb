use crate::domain::credentials::Credentials;
use crate::domain::users::hashed_user_password::HashedUserPassword;
use crate::domain::users::{UpdateUser, UserName, UserPassword};
use crate::services::{get_stored_credentials, get_user_by_name, update_user};
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

#[tracing::instrument(name = "Change password", skip(pool, password))]
pub async fn change_password(
    pool: &Pool,
    user_name: &UserName,
    password: &UserPassword,
) -> Result<(), anyhow::Error> {
    let conn = pool.get()?;
    let user = get_user_by_name(&conn, user_name)?
        .ok_or(anyhow::anyhow!("Failed to get user with given name"))?;
    let id = user.id;
    let password = HashedUserPassword::parse(&password, &user.password_salt);
    let changeset = UpdateUser {
        id,
        name: None,
        email: None,
        password: Some(password),
        password_salt: None,
        is_banned: None,
    };
    update_user(&conn, changeset)?;
    Ok(())
}
