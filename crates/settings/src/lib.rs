use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, path::PathBuf};
use storage::PoolConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub api: ApiSettings,
    pub database: PoolConfig,
    pub logging: LoggingSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let current_dir = env::current_dir().unwrap();
        Self::new_setting_path(current_dir.join("settings.yaml"))
    }

    pub fn new_setting_path(path: PathBuf) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::from(path))
            .add_source(
                Environment::with_prefix("")
                    .prefix_separator("")
                    .separator("_"),
            )
            .build()?;
        s.try_deserialize()
    }
}
