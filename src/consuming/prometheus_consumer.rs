use std::collections::HashMap;
use prometheus::{Registry, Gauge, Opts, TextEncoder, Encoder};
use crate::consuming::common::PullseConsumer;
use crate::PullseLedger;

pub struct PrometheusConsumer {
    registry: Registry,
    collectors: HashMap<String, Gauge>,
}

impl PrometheusConsumer {
    pub fn new(ledger: &PullseLedger) -> PrometheusConsumer {
        let registry = Registry::new();
        let mut collectors = HashMap::new();

        for metric_name in ledger.get_metric_names() {
            let gauge_opts = Opts::new(metric_name, metric_name);
            let gauge = Gauge::with_opts(gauge_opts).unwrap();

            registry.register(Box::new(gauge.clone())).unwrap();
            collectors.insert(String::from(metric_name), gauge);
        }
        PrometheusConsumer { registry, collectors }
    }

    pub fn report(&self) {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();
        println!("{}", result);
    }
}

impl PullseConsumer for PrometheusConsumer {
    fn consume(&self, ledger: &PullseLedger) {
        for metric_name in ledger.get_metric_names() {
            if let Some(collector) = self.collectors.get(metric_name) {
                if let Some(value) = ledger.raw_data.get(metric_name) {
                    collector.set(value.clone().into());
                }
            }
        }
    }
}
