use crate::settings::{AgentSettings, GathererKey};
use std::collections::HashMap;

// general traits, structs, etc
pub mod common;

// concrete implementations
pub mod weather;

pub fn get_gatherers(
    settings: &HashMap<GathererKey, AgentSettings>,
) -> Vec<Box<(dyn common::PullseGatherer + Send)>> {
    let mut result = Vec::new();

    if let Some(weather_settings) = settings.get("weather") {
        if weather_settings.enabled {
            result.push(
                Box::new(weather::WeatherDataGatherer::new(&weather_settings.options))
                    as Box<(dyn common::PullseGatherer + Send)>,
            );
        }
    }

    result
}
