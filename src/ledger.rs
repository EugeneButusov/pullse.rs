use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

// TODO: think about assoc types for hashmap
pub struct PullseLedger {
    raw_data: HashMap<String, f64>,
}

impl PullseLedger {
    pub fn new() -> Self {
        PullseLedger {
            raw_data: HashMap::new(),
        }
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

    pub fn get_metric(&self, key: &str) -> Option<&f64> {
        self.raw_data.get(key)
    }
}

impl Default for PullseLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for PullseLedger {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.raw_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_create() {
        let ledger = PullseLedger::new();
        assert_eq!(ledger.raw_data.len(), 0);
    }

    #[test]
    fn test_ledger_default() {
        let ledger = PullseLedger::default();
        assert_eq!(ledger.raw_data.len(), 0);
    }

    #[test]
    fn test_ledger_insert() {
        let mut ledger = PullseLedger::new();

        ledger.insert(("TEST_KEY".to_string(), 5.0));
        assert_eq!(ledger.raw_data.get("TEST_KEY").unwrap().clone(), 5.0);

        ledger.insert(("TEST_KEY".to_string(), 6.0));
        assert_eq!(ledger.raw_data.get("TEST_KEY").unwrap().clone(), 6.0);
    }

    #[test]
    fn test_ledger_get_metric_names() {
        let mut ledger = PullseLedger::new();

        let keys = vec!["TEST_KEY_1".to_string(), "TEST_KEY_2".to_string()];

        for key in &keys {
            ledger.insert((key.clone(), 5.0));
        }

        let metric_names = ledger.get_metric_names();

        assert_eq!(
            metric_names
                .iter()
                .filter(|item| keys
                    .iter()
                    .find(|key| (&key).eq(item)) != None
                )
                .count(),
            metric_names.len()
        );
    }
}
