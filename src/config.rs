use config::{Config, ConfigBuilder};
use secrecy::Secret;

#[derive(Clone, serde::Deserialize)]
pub struct HmacSecret(pub Secret<String>);

#[derive(Clone, serde::Deserialize)]
pub struct Settings {
    pub hmac_secret: HmacSecret,
    pub database_path: String,
    pub application_port: u16
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    config.try_deserialize()
}