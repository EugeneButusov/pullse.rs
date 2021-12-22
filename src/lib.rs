use std::collections::HashMap;

pub mod pulling;
pub mod consuming;

#[derive(Debug)]
pub struct PullseLedger {
    raw_data: HashMap<String, f32>,
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

    pub fn insert(&mut self, (key, value): (String, f32)) {
        self.raw_data.insert(key, value);
    }
}
