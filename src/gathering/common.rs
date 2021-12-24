use std::collections::HashMap;

pub trait PullseGatherer {
    fn gather(&self) -> HashMap<String, f64>;
}
