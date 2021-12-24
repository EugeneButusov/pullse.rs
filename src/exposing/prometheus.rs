use std::collections::HashMap;
use std::sync::Arc;
use prometheus::{Registry, Gauge, Opts, TextEncoder, Encoder};
use warp::Filter;
use tokio::runtime::Runtime;
use super::common::PullseExposer;
use super::PullseLedger;

pub struct PrometheusExposer {
    collectors: HashMap<String, Gauge>,
    tokio_runtime: Option<Runtime>,
}

impl PrometheusExposer {
    pub fn new(ledger: &PullseLedger) -> PrometheusExposer {
        let registry = Registry::new();
        let mut collectors = HashMap::new();

        for metric_name in ledger.get_metric_names() {
            let gauge_opts = Opts::new(metric_name, metric_name);
            let gauge = Gauge::with_opts(gauge_opts).unwrap();

            registry.register(Box::new(gauge.clone())).unwrap();
            collectors.insert(String::from(metric_name), gauge);
        }

        let registry = Arc::new(registry);

        let mut result = PrometheusExposer {
            collectors,
            tokio_runtime: None,
        };

        let gathering_registry = Arc::clone(&registry);
        let metrics_taker = warp::path("metrics").map(move || {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = gathering_registry.gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            let result = String::from_utf8(buffer).unwrap();
            result
        });

        let rt = Runtime::new()
            .unwrap();

        rt.spawn(warp::serve(metrics_taker).run(([127, 0, 0, 1], 3030)));

        result.tokio_runtime = Some(rt);

        result
    }
}

impl PullseExposer for PrometheusExposer {
    fn consume(&self, ledger: &PullseLedger) {
        for metric_name in ledger.get_metric_names() {
            if let Some(collector) = self.collectors.get(metric_name) {
                if let Some(value) = ledger.get_metric(metric_name) {
                    collector.set(value.clone().into());
                }
            }
        }
    }
}
