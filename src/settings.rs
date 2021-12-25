use std::env;
use config::{ConfigError, Config, File, Environment};
use serde::Deserializer;

pub struct Settings {

}

impl Settings {
    pub fn new(source_path: String) -> Result<Config, ConfigError> {
        let mut config = Config::default();

        // Start off by merging in the "default" configuration file
        config.merge(File::with_name("config/default"))?;

        // Add in a custom configuration file
        config.merge(File::with_name(source_path.as_str()).required(false))?;

        // Now that we're done, let's access our configuration
        if let Ok(t) = config.get_table("gatherers.weather") {
            if let Some(val) = t.get("location") {
                println!("{:?}", val.kind);
            }
        }

        // You can deserialize (and thus freeze) the entire configuration as
        Ok(config)
    }
}
