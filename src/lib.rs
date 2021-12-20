use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug)]
pub struct Registry {
    raw_data: HashMap<String, f32>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry { raw_data: HashMap::new() }
    }

    pub fn insert(&mut self, (key, value): (String, f32)) {
        self.raw_data.insert(key, value);
    }
}

pub struct DataPuller {}

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

impl DataPuller {
    pub fn new() -> DataPuller {
        DataPuller {}
    }

    pub fn pull_data(&self) -> HashMap<String, f32> {
        let mut result = HashMap::new();

        let resp = reqwest::blocking::get("http://api.weatherapi.com/v1/current.json?key=%%API_KEY%%&q=%%LOCATION%%&aqi=no")
            .unwrap()
            .json::<WeatherData>().unwrap();

        result.insert(String::from("LOCAL_TEMPERATURE"), resp.current.temp_c);

        result
    }
}
