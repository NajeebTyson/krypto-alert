use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub poll_interval: i16,
}

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub level: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub logging: Logging,
    pub settings: Settings,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::default();
        config.merge(File::with_name("Settings"))?;
        config.try_into()
    }
}

lazy_static! {
    pub static ref SETTINGS: AppConfig = get_config();
}

/// Load env into Config struct
fn get_config() -> AppConfig {
    match AppConfig::new() {
        Ok(config_) => config_,
        Err(err) => panic!("Configuration Error!: {:#?}", err),
    }
}
