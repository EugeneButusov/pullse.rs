// general traits, structs, etc
pub mod common;

// concrete implementations
pub mod weather;

pub fn get_gatherers() -> Vec<Box<(dyn common::PullseGatherer + Send)>> {
    let mut result = Vec::new();

    result.push(Box::new(weather::WeatherDataGatherer::new()) as Box<(dyn common::PullseGatherer + Send)>);

    result
}
