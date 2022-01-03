use std::collections::HashMap;
use config::Value;
use super::PullseLedger;

pub trait PullseExposer {
    fn new(ledger: &PullseLedger, settings: &HashMap<String, Value>) -> Result<Self, ()> where Self: Sized;
    fn consume(&self, ledger: &PullseLedger);
}
