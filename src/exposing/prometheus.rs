use super::common::{ExposerInitError, PullseExposer};
use super::PullseLedger;
use config::Value;
use log::error;
use prometheus::{Encoder, Gauge, Opts, Registry, TextEncoder};
use std::collections::HashMap;
use tokio::runtime::Runtime;
use warp::Filter;

pub struct PrometheusExposer {
    collectors: HashMap<String, Gauge>,
}

impl PullseExposer for PrometheusExposer {
    fn new(
        ledger: &PullseLedger,
        settings: &HashMap<String, Value>,
    ) -> Result<Self, ExposerInitError> {
        let port: u16 = match settings.get("port") {
            Some(val) => match val.clone().try_into() {
                Ok(val) => val,
                Err(_) => {
                    return Err(ExposerInitError::SettingBadType(
                        String::from("port"),
                        String::from(std::any::type_name::<u16>()),
                    ))
                }
            },
            None => return Err(ExposerInitError::SettingUndefined(String::from("port"))),
        };

        let registry = Registry::new();
        let mut collectors = HashMap::new();

        for metric_name in ledger.get_metric_names() {
            let gauge_opts = Opts::new(metric_name, metric_name);
            let gauge = match Gauge::with_opts(gauge_opts) {
                Ok(val) => val,
                Err(_) => {
                    return Err(ExposerInitError::Other(format!(
                        "Unable to create gauge for {}",
                        metric_name
                    )))
                }
            };

            match registry.register(Box::new(gauge.clone())) {
                Ok(_) => {}
                Err(_) => {
                    return Err(ExposerInitError::Other(format!(
                        "Unable to register gauge for {}",
                        metric_name
                    )))
                }
            };
            collectors.insert(String::from(metric_name), gauge);
        }

        let metrics_taker = warp::path("metrics").map(move || {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = registry.gather();
            match encoder.encode(&metric_families, &mut buffer) {
                Ok(_) => {}
                Err(error) => {
                    error!("Cannot encode Prometheus data: {:?}", error);
                }
            };
            String::from_utf8(buffer).unwrap()
        });

        let rt = match Runtime::new() {
            Ok(val) => val,
            Err(error) => {
                return Err(ExposerInitError::Other(String::from(format!(
                    "Unable to instantiate tokio runtime: {}",
                    error
                ))))
            }
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
