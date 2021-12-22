use std::env;
use std::collections::HashMap;
use serde::Deserialize;

use crate::pulling::common::DataPuller;

pub struct WeatherDataPuller {
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

impl WeatherDataPuller {
    pub fn new() -> WeatherDataPuller {
        let api_key = env::var("WEATHER_PULLER_API_KEY").unwrap();
        let location = env::var("WEATHER_PULLER_LOCATION").unwrap();

        WeatherDataPuller {
            api_key,
            location,
        }
    }
}

impl DataPuller for WeatherDataPuller {
    fn pull_data(&self) -> HashMap<String, f32> {
        let mut result = HashMap::new();

        let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", self.api_key, self.location);
        let resp = reqwest::blocking::get(url)
            .unwrap()
            .json::<WeatherData>().unwrap();

        result.insert(String::from("LOCAL_TEMPERATURE"), resp.current.temp_c);

        result
    }
}
