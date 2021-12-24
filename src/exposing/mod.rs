use super::ledger::*;

// general traits, structs, etc
pub mod common;

// concrete implementations
pub mod prometheus;

pub fn get_exposers(ledger: &PullseLedger) -> Vec<Box<(dyn common::PullseExposer + Send)>> {
    let mut result = Vec::new();

    result.push(Box::new(prometheus::PrometheusExposer::new(ledger)) as Box<(dyn common::PullseExposer + Send)>);

    result
}
