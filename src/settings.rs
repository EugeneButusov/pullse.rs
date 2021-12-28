use std::collections::HashMap;
use config::{ConfigError, Config, File, Value};

const DEFAULT_CONFIG_PATH: &str = "/usr/local/share/pullse/config/default";

#[derive(Debug, Deserialize)]
pub struct CommonSettings {
    pub pull_timeout: u64,
}

pub type ExposerKey = String;
pub type GathererKey = String;

#[derive(Debug, Deserialize)]
pub struct AgentSettings {
    pub enabled: bool,
    pub options: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub common: CommonSettings,
    pub gatherers: HashMap<GathererKey, AgentSettings>,
    pub exposers: HashMap<ExposerKey, AgentSettings>,
}

impl Settings {
    pub fn new_default() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.merge(File::with_name(DEFAULT_CONFIG_PATH))?;

        s.try_into()
    }

    pub fn new_from_custom_config(custom_config_path: String) -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.merge(File::with_name(DEFAULT_CONFIG_PATH).required(false))?;
        s.merge(File::with_name(custom_config_path.as_str()))?;

        s.try_into()
    }
}
