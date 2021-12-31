use super::common::PullseExposer;
use super::PullseLedger;
use config::Value;
use prometheus::{Encoder, Gauge, Opts, Registry, TextEncoder};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use warp::Filter;

pub struct PrometheusExposer {
    collectors: HashMap<String, Gauge>,
    tokio_runtime: Option<Runtime>,
}

impl PrometheusExposer {
    pub fn new(ledger: &PullseLedger, settings: &HashMap<String, Value>) -> PrometheusExposer {
        // TODO: think about .unwrap().clone() is good chaining
        let port: u16 = settings
            .get("port")
            .unwrap()
            .clone()
            .try_into()
            .expect("PrometheusExposer::new -> `port` should be a number.");

        let registry = Registry::new();
        let mut collectors = HashMap::new();

        for metric_name in ledger.get_metric_names() {
            let gauge_opts = Opts::new(metric_name, metric_name);
            let gauge = Gauge::with_opts(gauge_opts).unwrap();

            registry.register(Box::new(gauge.clone())).unwrap();
            collectors.insert(String::from(metric_name), gauge);
        }

        let registry = Arc::new(registry);

        // TODO: these twists around tokio runtime do not look fine, need to avoid approaches mixin
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
            String::from_utf8(buffer).unwrap()
        });

        let rt = Runtime::new().unwrap();

        rt.spawn(warp::serve(metrics_taker).run(([0, 0, 0, 0], port)));

        result.tokio_runtime = Some(rt);

        result
    }
}

impl PullseExposer for PrometheusExposer {
    fn consume(&self, ledger: &PullseLedger) {
        for metric_name in ledger.get_metric_names() {
            if let Some(collector) = self.collectors.get(metric_name) {
                if let Some(value) = ledger.get_metric(metric_name) {
                    collector.set(*value);
                }
            }
        }
    }
}
