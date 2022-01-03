use std::collections::HashMap;
use config::Value;
use prometheus::{Encoder, Gauge, Opts, Registry, TextEncoder};
use tokio::runtime::Runtime;
use warp::Filter;
use super::common::{PullseExposer, ExposerInitError};
use super::PullseLedger;

pub struct PrometheusExposer {
    collectors: HashMap<String, Gauge>,
}

impl PullseExposer for PrometheusExposer {
    fn new(ledger: &PullseLedger, settings: &HashMap<String, Value>) -> Result<Self, ExposerInitError> {
        let port: u16 = match settings.get("port") {
            Some(val) => {
                match val.clone().try_into() {
                    Ok(val) => val,
                    Err(_) => return Err(ExposerInitError::SettingBadType(String::from("port"), String::from(std::any::type_name::<u16>()))),
                }
            },
            None => return Err(ExposerInitError::SettingUndefined(String::from("port"))),
        };

        let registry = Registry::new();
        let mut collectors = HashMap::new();

        for metric_name in ledger.get_metric_names() {
            let gauge_opts = Opts::new(metric_name, metric_name);
            let gauge = Gauge::with_opts(gauge_opts).unwrap();

            registry.register(Box::new(gauge.clone())).unwrap();
            collectors.insert(String::from(metric_name), gauge);
        }

        let metrics_taker = warp::path("metrics").map(move || {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = registry.gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        });

        let rt = match Runtime::new() {
            Ok(val) => val,
            Err(error) => return Err(ExposerInitError::Other(String::from(format!("Unable to instantiate tokio runtime: {}", error))))
        };

        rt.spawn(warp::serve(metrics_taker).run(([0, 0, 0, 0], port)));

        Ok(PrometheusExposer { collectors })
    }

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
