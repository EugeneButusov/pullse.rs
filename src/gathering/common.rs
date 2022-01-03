use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use config::Value;

pub enum GathererInitError {
    SettingUndefined(String), // setting key name
    SettingBadType(String, String), // setting key name and required type
    Other(String), // message
}

impl Debug for GathererInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GathererInitError::SettingUndefined(key) => {
                write!(f, "GathererInitError: Setting `{:?}` is not defined", key)
            }
            GathererInitError::SettingBadType(key, type_name) => {
                write!(f, "GathererInitError: Setting `{:?}` should have `{:?}` type", key, type_name)
            }
            GathererInitError::Other(message) => {
                write!(f, "GathererInitError: {:?}", message)
            }
        }
    }
}

impl Display for GathererInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GathererInitError::SettingUndefined(key) => {
                write!(f, "ExposerInitError: Setting with key `{}` is not defined", key)
            }
            GathererInitError::SettingBadType(key, type_name) => {
                write!(f, "ExposerInitError: Setting with key `{}` needs type `{}`", key, type_name)
            }
            GathererInitError::Other(message) => {
                write!(f, "ExposerInitError: {}", message)
            }
        }
    }
}

impl Error for GathererInitError {
}

pub trait PullseGatherer {
    fn new(settings: &HashMap<String, Value>) -> Result<Self, GathererInitError> where Self: Sized;
    fn gather(&self) -> HashMap<String, f64>;
}
