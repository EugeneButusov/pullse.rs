use std::collections::HashMap;

pub struct PullseLedger {
    raw_data: HashMap<String, f64>,
}

impl PullseLedger {
    pub fn new() -> PullseLedger {
        PullseLedger { raw_data: HashMap::new() }
    }

    pub fn get_metric_names(&self) -> Vec<&String> {
        let mut result = Vec::new();

        for (key, _) in self.raw_data.iter() {
            result.push(key);
        }

        result
    }

    pub fn insert(&mut self, (key, value): (String, f64)) {
        self.raw_data.insert(key, value);
    }

    pub fn get_metric(&self, key: &String) -> Option<&f64> {
        self.raw_data.get(key)
    }
}
