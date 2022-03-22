use secrecy::Secret;

#[derive(Clone, serde::Deserialize)]
pub struct HmacSecret(Secret<String>);

#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub hmac_secret: HmacSecret
}
