use crate::settings::{AgentSettings, GathererKey};
use common::PullseGatherer;
use log::error;
use std::collections::HashMap;

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
                Ok(weather_gatherer) => {
                    result
                        .push(Box::new(weather_gatherer)
                            as Box<dyn common::PullseGatherer + Sync + Send>);
                }
                Err(error) => {
                    error!("Unable to instantiate WeatherDataGatherer: {}", error);
                }
            }
        }
    }

    result
}
