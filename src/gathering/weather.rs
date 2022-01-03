use config::Value;
use serde::Deserialize;
use std::collections::HashMap;
use super::common::{PullseGatherer, GathererInitError};

static LOCAL_TEMPERATURE_KEY: &str = "LOCAL_TEMPERATURE";

pub struct WeatherDataGatherer {
    api_key: String,
    location: String,
}

#[derive(Deserialize, Debug)]
struct WeatherCondition {
    temp_c: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    current: WeatherCondition,
}

impl PullseGatherer for WeatherDataGatherer {
    fn new(settings: &HashMap<String, Value>) -> Result<Self, GathererInitError> {
        let api_key: String = settings
            .get("api_key")
            .expect("WeatherDataGatherer::new -> `api_key` is not defined")
            .clone()
            .try_into()
            .expect("WeatherDataGatherer::new -> `api_key` should be a string");
        let location: String = settings
            .get("location")
            .expect("WeatherDataGatherer::new -> `location` is not defined")
            .clone()
            .try_into()
            .expect("WeatherDataGatherer::new -> `location` should be a string");

        // TODO: may be better to use Result Error instead of panicking
        if api_key.chars().count() == 0 {
            panic!("WeatherDataGatherer::new -> api_key cannot be empty");
        }

        if location.chars().count() == 0 {
            panic!("WeatherDataGatherer::new -> location cannot be empty");
        }

        Ok(WeatherDataGatherer { api_key, location })
    }

    fn gather(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();

        let url = format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            self.api_key, self.location
        );
        let resp = reqwest::blocking::get(url)
            .unwrap()
            .json::<WeatherData>()
            .unwrap();

        result.insert(
            String::from(LOCAL_TEMPERATURE_KEY),
            resp.current.temp_c.into(),
        );

        result
    }
}
