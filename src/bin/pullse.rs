use log::{debug, info};
use pullse::exposing::get_exposers;
use pullse::gathering::get_gatherers;
use pullse::ledger::PullseLedger;
use pullse::settings::Settings;
use pullse::app::App;
use simple_logger::SimpleLogger;
use std::sync::mpsc::channel;
use std::{env, thread, time};

fn main() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .env()
        .init()
        .unwrap();

    let settings = if let Ok(custom_config_path) = env::var("CONFIG_PATH") {
        Settings::new_from_custom_config(custom_config_path)
    } else {
        Settings::new_default()
    }.expect("Config cannot be read as it's corrupted");

    App::new(settings).run();
}
