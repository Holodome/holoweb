use secrecy::{ExposeSecret, Secret};

/// Settings related to application
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    /// Port where to start server. If 0, will be chosen randomly by OS
    pub port: u16,
    /// Host path. Typically 127.0.0.1
    pub host: String,
    /// Secret that is used to make HMAC
    pub hmac_secret: Secret<String>,
    /// Number of worker threads.
    /// If not specified, number of CPU cores is used
    pub workers: Option<usize>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DbConfig {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    #[serde(skip, default)]
    pub in_memory: bool,
    #[serde(skip, default)]
    pub run_migrations: bool,
}

impl DbConfig {
    pub fn uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.username,
            &self.password.expose_secret(),
            &self.host,
            self.port,
            &self.database_name
        )
    }

    pub fn uri_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            &self.username,
            &self.password.expose_secret(),
            &self.host,
            self.port
        )
    }
}

/// Settings of whole system
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub database: DbConfig,
    /// Settings of application
    pub app: AppConfig,
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
pub fn get_config() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get current directory");
    let config_dir = base_path.join("config");

    let env: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to set app environment");

    let config = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base")).required(true))
        .add_source(config::File::from(config_dir.join(env.as_str())).required(true))
        // Environment variables with prefix APP__ override settings loaded from files.
        // For example APP__APP__PORT=8081 will override port setting.
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?;

    config.try_deserialize()
}
