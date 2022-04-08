use config::Config;
use secrecy::Secret;

/// Settings related to application
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    /// Port where to start server. If 0, will be chosen randomly by OS
    pub port: u16,
    /// Host path. Typically 127.0.0.1
    pub host: String,
    /// Secret that is used to make HMAC
    pub hmac_secret: Secret<String>,
}

/// Settings of whole system
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    /// URI of database, typically path to database file
    pub database_path: String,
    /// Settings of application
    pub app: ApplicationSettings,
    /// Redis URI. Typically address of redis hosted in docker container
    pub redis_uri: Secret<String>,
}

/// Environment in which application is running.
/// Used to tell server to use different config file. (local.yml or production.yml)
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

/// Gets config from local settings files.
pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get current directory");
    let config_dir = base_path.join("config");

    let env: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to set app environment");

    let config = Config::builder()
        .add_source(config::File::from(config_dir.join("base")).required(true))
        .add_source(config::File::from(config_dir.join(env.as_str())).required(true))
        .build()?;

    config.try_deserialize()
}
