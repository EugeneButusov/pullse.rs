use super::common::{GathererInitError, PullseGatherer};
use config::Value;
use log::error;
use serde::Deserialize;
use std::collections::HashMap;

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
        let api_key: String = match settings.get("api_key") {
            Some(val) => match val.clone().try_into() {
                Ok(val) => val,
                Err(_) => {
                    return Err(GathererInitError::SettingBadType(
                        String::from("api_key"),
                        String::from(std::any::type_name::<String>()),
                    ))
                }
            },
            None => return Err(GathererInitError::SettingUndefined(String::from("api_key"))),
        };

        let location: String = match settings.get("location") {
            Some(val) => match val.clone().try_into() {
                Ok(val) => val,
                Err(_) => {
                    return Err(GathererInitError::SettingBadType(
                        String::from("location"),
                        String::from(std::any::type_name::<String>()),
                    ))
                }
            },
            None => {
                return Err(GathererInitError::SettingUndefined(String::from(
                    "location",
                )))
            }
        };

        if api_key.chars().count() == 0 {
            return Err(GathererInitError::Other(String::from(
                "api_key cannot be empty",
            )));
        }

        if location.chars().count() == 0 {
            return Err(GathererInitError::Other(String::from(
                "location cannot be empty",
            )));
        }

        Ok(WeatherDataGatherer { api_key, location })
    }

    fn gather(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();

        let url = format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            self.api_key, self.location
        );
        let weather_data: Option<WeatherData> = match reqwest::blocking::get(url) {
            Ok(response) => match response.json::<WeatherData>() {
                Ok(data) => Some(data),
                Err(parse_error) => {
                    error!("Error occurred parsing weather data: {}", parse_error);
                    None
                }
            },
            Err(reqwest_error) => {
                error!("Error occurred during http request: {}", reqwest_error);
                None
            }
        };

        if let Some(weather_data) = weather_data {
            result.insert(
                String::from(LOCAL_TEMPERATURE_KEY),
                weather_data.current.temp_c.into(),
            );
        }

        result
    }
}
