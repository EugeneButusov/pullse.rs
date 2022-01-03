use std::collections::HashMap;
use config::Value;

pub trait PullseGatherer {
    fn new(settings: &HashMap<String, Value>) -> Result<Self, ()> where Self: Sized;
    fn gather(&self) -> HashMap<String, f64>;
}
