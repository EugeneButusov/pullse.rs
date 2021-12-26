use std::collections::HashMap;
use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct CommonSettings {
    pub pull_timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct GathererSettings {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct ExposerSettings {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub common: CommonSettings,
    pub gatherers: HashMap<String, GathererSettings>,
    pub exposers: HashMap<String, ExposerSettings>,
}

impl Settings {
    pub fn new_default() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.merge(File::with_name("config/default"))?;

        s.try_into()
    }

    pub fn new_from_custom_config(custom_config_path: String) -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.merge(File::with_name("config/default"))?;
        s.merge(File::with_name(custom_config_path.as_str()).required(false))?;

        s.try_into()
    }
}
