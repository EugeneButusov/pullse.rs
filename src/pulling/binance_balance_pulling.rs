use std::collections::HashMap;
use std::env;
use binance::api::*;
use binance::account::*;
use crate::pulling::common::DataPuller;

pub struct BinanceBalancePuller {
    account: Account,
}

impl BinanceBalancePuller {
    pub fn new() -> BinanceBalancePuller {
        let api_key = Some(env::var("BINANCE_PULLER_API_KEY").unwrap());
        let secret_key = Some(env::var("BINANCE_PULLER_SECRET_KEY").unwrap());

        let account: Account = Binance::new(api_key, secret_key);
        BinanceBalancePuller{ account }
    }
}

impl DataPuller for BinanceBalancePuller {
    fn pull_data(&self) -> HashMap<String, f32> {
        let mut result = HashMap::new();

        let answer = self.account.get_account().unwrap();
        println!("{:?}", answer.balances);

        result.insert(String::from("BINANCE_BALANCE"), 0.003);

        result
    }
}
