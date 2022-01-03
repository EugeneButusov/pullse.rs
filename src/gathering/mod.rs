use std::collections::HashMap;
use log::error;
use crate::settings::{AgentSettings, GathererKey};

// general traits, structs, etc
pub mod common;

// concrete implementations
pub mod weather;

pub fn get_gatherers(
    settings: &HashMap<GathererKey, AgentSettings>,
) -> Vec<Box<dyn common::PullseGatherer + Sync + Send>> {
    let mut result = Vec::new();

    if let Some(weather_settings) = settings.get("weather") {
        if weather_settings.enabled {
            match weather::WeatherDataGatherer::new(&weather_settings.options) {
                Ok(weatherGatherer) => {
                    result.push(
                        Box::new(weatherGatherer) as Box<dyn common::PullseGatherer + Sync + Send>,
                    );
                }
                Err(error) => {
                    error!("Unable to instantiate WeatherDataGatherer: {}", error);
                }
            }
        }
    }

    result
}
