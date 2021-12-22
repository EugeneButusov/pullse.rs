use prometheus::{Registry, Gauge, Opts};
use crate::consuming::common::PullseConsumer;
use crate::PullseLedger;

pub struct PrometheusConsumer {
    registry: Registry,
}

impl PrometheusConsumer {
    fn new(ledger: &PullseLedger) -> PrometheusConsumer {
        let registry = Registry::new();

        for metric_name in ledger.get_metric_names() {
            let gauge_opts = Opts::new(metric_name, metric_name);
            let counter = Gauge::with_opts(gauge_opts).unwrap();

            registry.register(Box::new(counter.clone())).unwrap();
        }
        PrometheusConsumer { registry }
    }
}

impl PullseConsumer for PrometheusConsumer {
    fn consume(&self, ledger: &PullseLedger) {
        todo!("consume data")
    }
}
