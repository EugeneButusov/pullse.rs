use super::ledger::*;
use crate::settings::{AgentSettings, ExposerKey};
use std::collections::HashMap;

// general traits, structs, etc
pub mod common;

// concrete implementations
pub mod prometheus;

pub fn get_exposers(
    ledger: &PullseLedger,
    settings: &HashMap<ExposerKey, AgentSettings>,
) -> Vec<Box<(dyn common::PullseExposer + Send)>> {
    let mut result = Vec::new();

    if let Some(prometheus_settings) = settings.get("prometheus") {
        if prometheus_settings.enabled {
            result.push(Box::new(prometheus::PrometheusExposer::new(
                ledger,
                &prometheus_settings.options,
            )) as Box<(dyn common::PullseExposer + Send)>);
        }
    }

    result
}
