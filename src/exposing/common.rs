use crate::exposing::prometheus::PrometheusExposer;
use crate::PullseLedger;

pub trait PullseExposer {
    fn consume(&self, ledger: &PullseLedger);
}

pub fn get_consumers(ledger: &PullseLedger) -> Vec<Box<(dyn PullseExposer + Send)>> {
    let mut result = Vec::new();

    result.push(Box::new(PrometheusExposer::new(ledger)) as Box<(dyn PullseExposer + Send)>);

    result
}
