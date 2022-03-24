use config::Config;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub database_path: String,
    pub application_port: u16,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    config.try_deserialize()
}
