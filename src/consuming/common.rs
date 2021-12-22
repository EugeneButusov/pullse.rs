use crate::consuming::prometheus_consumer::PrometheusConsumer;
use crate::PullseLedger;

pub trait PullseConsumer {
    fn consume(&self, ledger: &PullseLedger);
}

pub fn get_consumers(ledger: &PullseLedger) -> Vec<Box<(dyn PullseConsumer + Send)>> {
    let mut result = Vec::new();

    result.push(Box::new(PrometheusConsumer::new(ledger)) as Box<(dyn PullseConsumer + Send)>);

    result
}
