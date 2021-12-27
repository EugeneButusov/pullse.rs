use std::collections::HashMap;
use config::Value;
use serde::Deserialize;

use super::common::PullseGatherer;

pub struct WeatherDataGatherer {
    api_key: String,
    location: String,
}

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
    tz_id: String,
    localtime_epoch: u64,
    localtime: String
}

#[derive(Deserialize, Debug)]
struct WeatherCondition {
    last_updated_epoch: u64,
    last_updated: String,
    temp_c: f32,
    condition: HashMap<String, String>,
    wind_kph: f32,
    uv: f32
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    location: Location,
    current: WeatherCondition,
}

impl WeatherDataGatherer {
    pub fn new(settings: &HashMap<String, Value>) -> WeatherDataGatherer {
        // TODO: think about .unwrap().clone() is good chaining
        let api_key: String = settings.get("api_key").unwrap().clone().try_into()
            .expect("WeatherDataGatherer::new -> api_key should be a string.");
        let location: String = settings.get("location").unwrap().clone().try_into()
            .expect("WeatherDataGatherer::new -> location should be a string.");

        // TODO: may be better to use Result Error instead of panicking
        if api_key.chars().count() == 0 {
            panic!("WeatherDataGatherer::new -> api_key cannot be empty.");
        }

        if location.chars().count() == 0 {
            panic!("WeatherDataGatherer::new -> location cannot be empty.");
        }

        WeatherDataGatherer {
            api_key,
            location,
        }
    }
}

impl PullseGatherer for WeatherDataGatherer {
    fn gather(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();

        let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", self.api_key, self.location);
        let resp = reqwest::blocking::get(url)
            .unwrap()
            .json::<WeatherData>().unwrap();

        result.insert(String::from("LOCAL_TEMPERATURE"), resp.current.temp_c.into());

        result
    }
}
