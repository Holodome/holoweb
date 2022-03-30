use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct User {
    id: String,
    name: String,
    password: Secret<String>,
}
