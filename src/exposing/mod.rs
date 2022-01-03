use log::error;
use std::collections::HashMap;
use common::PullseExposer;
use super::ledger::*;
use crate::settings::{AgentSettings, ExposerKey};

// general traits, structs, etc
pub mod common;

// concrete implementations
pub mod prometheus;

pub fn get_exposers(
    ledger: &PullseLedger,
    settings: &HashMap<ExposerKey, AgentSettings>,
) -> Vec<Box<dyn common::PullseExposer + Send + Sync>> {
    let mut result = Vec::new();

    if let Some(prometheus_settings) = settings.get("prometheus") {
        if prometheus_settings.enabled {
            match prometheus::PrometheusExposer::new(ledger, &prometheus_settings.options) {
                Ok(prometheus_exposer) => {
                    result.push(Box::new(prometheus_exposer) as Box<dyn common::PullseExposer + Send + Sync>);
                }
                Err(error) => {
                    error!("Unable to instantiate WeatherDataGatherer: {}", error);
                }
            }
        }
    }

    result
}
