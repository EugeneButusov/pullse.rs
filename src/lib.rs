use std::collections::HashMap;

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

impl DataPuller {
    pub fn new() -> DataPuller {
        DataPuller {}
    }

    pub fn pull_data(&self) -> HashMap<String, f32> {
        let mut result = HashMap::new();

        result.insert(String::from("MY_AWESOME_METRIC"), 0.5);

        result
    }
}
