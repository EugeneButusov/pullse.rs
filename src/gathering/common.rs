use std::collections::HashMap;
use crate::gathering::weather::WeatherDataGatherer;

pub trait PullseGatherer {
    fn pull_data(&self) -> HashMap<String, f32>;
}

pub fn get_gatherers() -> Vec<Box<(dyn PullseGatherer + Send)>> {
    let mut result = Vec::new();

    result.push(Box::new(WeatherDataGatherer::new()) as Box<(dyn PullseGatherer + Send)>);

    result
}
