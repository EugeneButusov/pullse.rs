use std::collections::HashMap;

use crate::pulling::common::DataPuller;

pub struct BinanceBalancePuller {}

impl BinanceBalancePuller {
    pub fn new() -> BinanceBalancePuller {
        BinanceBalancePuller{}
    }
}

impl DataPuller for BinanceBalancePuller {
    fn pull_data(&self) -> HashMap<String, f32> {
        let mut result = HashMap::new();

        result.insert(String::from("BINANCE_BALANCE"), 0.003);

        result
    }
}
