use config::Config;
use secrecy::Secret;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub hmac_secret: Secret<String>
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub database_path: String,
    pub app: ApplicationSettings,
    pub redis_uri: Secret<String>
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use 'local' or 'production'",
                other
            )),
        }
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get current directory");
    let config_dir = base_path.join("config");

    let env: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to get APP_ENV");

    let config = Config::builder()
        .add_source(config::File::from(config_dir.join("base")).required(true))
        .add_source(config::File::from(config_dir.join(env.as_str())).required(true))
        .build()?;

    config.try_deserialize()
}
