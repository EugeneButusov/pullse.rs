use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
pub struct Registry {
    raw_data: HashMap<String, String>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry { raw_data: HashMap::new() }
    }

    pub fn insert(&mut self, update: &HashMap<String, String>) {
        for (k, v) in update {
            self.raw_data.insert(k.to_string(), v.to_string());
        }
    }
}

pub struct DataPuller {}

impl DataPuller {
    pub fn new() -> DataPuller {
        DataPuller {}
    }

    pub fn pull_data(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        let mut rng = rand::thread_rng();

        let value = format!("MY AWESOME VALUE: {}", rng.gen::<u8>());
        result.insert(String::from("MY_AWESOME_METRIC"), value);

        result
    }
}