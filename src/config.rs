use secrecy::Secret;

#[derive(Clone, serde::Deserialize)]
pub struct HmacSecret(pub Secret<String>);

#[derive(Clone, serde::Deserialize)]
pub struct AppSettings {
    pub hmac_secret: HmacSecret,
}
