use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use config::Value;
use super::PullseLedger;

pub enum ExposerInitError {
    SettingUndefined(String), // setting key name
    SettingBadType(String, String), // setting key name and required type
    Other(String), // message
}

impl Debug for ExposerInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExposerInitError::SettingUndefined(key) => {
                write!(f, "ExposerInitError: Setting with key `{:?}` is not defined", key)
            }
            ExposerInitError::SettingBadType(key, type_name) => {
                write!(f, "ExposerInitError: Setting with key `{:?}` needs type `{:?}`", key, type_name)
            }
            ExposerInitError::Other(message) => {
                write!(f, "ExposerInitError: {:?}", message)
            }
        }
    }
}

impl Display for ExposerInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExposerInitError::SettingUndefined(key) => {
                write!(f, "ExposerInitError: Setting `{}` is not defined", key)
            }
            ExposerInitError::SettingBadType(key, type_name) => {
                write!(f, "ExposerInitError: Setting `{}` should have `{}` type", key, type_name)
            }
            ExposerInitError::Other(message) => {
                write!(f, "ExposerInitError: {}", message)
            }
        }
    }
}

impl Error for ExposerInitError {
}

pub trait PullseExposer {
    fn new(ledger: &PullseLedger, settings: &HashMap<String, Value>) -> Result<Self, ExposerInitError> where Self: Sized;
    fn consume(&self, ledger: &PullseLedger);
}
