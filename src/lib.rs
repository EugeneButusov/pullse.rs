use std::collections::HashMap;

#[derive(Debug)]
pub struct Registry<'a> {
    raw_data: HashMap<&'a str, &'a str>,
}

impl Registry<'_> {
    pub fn new<'a>() -> Registry<'a> {
        Registry { raw_data: HashMap::new() }
    }

    pub fn insert<'a>(&mut self, update: HashMap<&'a str, &'a str>) {
        for (k, v) in update {
            self.raw_data.insert(k, v);
        }
    }
}

pub struct DataPuller {}

impl DataPuller {
    pub fn new() -> DataPuller {
        DataPuller {}
    }

    pub fn pull_data(&self) -> HashMap<&str, &str> {
        let mut result = HashMap::new();

        result.insert("MY_AWESOME_METRIC", "MY_AWESOME_VALUE");

        result
    }
}
