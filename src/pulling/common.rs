use std::collections::HashMap;
use crate::pulling::weather_data_pulling::WeatherDataPuller;

pub trait PullsePuller {
    fn pull_data(&self) -> HashMap<String, f32>;
}

pub fn get_pullers() -> Vec<Box<(dyn PullsePuller + Send)>> {
    let mut result = Vec::new();

    result.push(Box::new(WeatherDataPuller::new()) as Box<(dyn PullsePuller + Send)>);

    result
}
