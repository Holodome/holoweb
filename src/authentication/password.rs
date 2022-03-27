use secrecy::Secret;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[source] anyhow::Error),
}

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}
