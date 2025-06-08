use color_eyre::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

const DEFAULT_CONFIG: &str = include_str!("../default-config.toml");

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerConfig {
    pub use_key_release: bool,
    pub freeze_time: f64,
    pub display_decimal_points: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub tickrate: f64,
    pub timer: TimerConfig,
}
impl AppConfig {
    /// Loads the application's configuration file.
    /// First searches for a valid, standard location (dependent on OS).
    /// If not found, will create a config file and load default values.
    pub fn load() -> Result<Self> {
        if let Some(project_dirs) = ProjectDirs::from("", "", "cubegen") {
            let config_path = project_dirs.config_dir().join("config.toml");

            if !config_path.exists() {
                fs::create_dir_all(config_path.parent().unwrap())?;
                fs::write(config_path, DEFAULT_CONFIG)?;
                return Self::load_default();
            }

            let config_data = fs::read_to_string(config_path)?;
            let config: AppConfig = toml::from_str(&config_data)?;

            return Ok(config);
        }

        // TODO: No valid project dirs, log warning here

        Self::load_default()
    }

    fn load_default() -> Result<Self> {
        let config: AppConfig = toml::from_str(DEFAULT_CONFIG)?;
        Ok(config)
    }
}
