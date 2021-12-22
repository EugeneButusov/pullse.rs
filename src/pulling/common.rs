use std::collections::HashMap;
use crate::pulling::binance_balance_pulling::BinanceBalancePuller;
use crate::pulling::weather_data_pulling::WeatherDataPuller;

pub trait DataPuller {
    fn pull_data(&self) -> HashMap<String, f32>;
}

pub fn get_pullers() -> Vec<Box<(dyn DataPuller + Send)>> {
    let mut result = Vec::new();

    result.push(Box::new(WeatherDataPuller::new()) as Box<(dyn DataPuller + Send)>);
    result.push(Box::new(BinanceBalancePuller::new()) as Box<(dyn DataPuller + Send)>);

    result
}
