use std::{env, thread, time};
use std::sync::mpsc::channel;
use log::{debug, info};
use simple_logger::SimpleLogger;
use pullse::ledger::{PullseLedger};
use pullse::gathering::get_gatherers;
use pullse::exposing::get_exposers;
use pullse::settings::Settings;

fn main() {
    SimpleLogger::new().with_utc_timestamps().env().init().unwrap();
    info!("Bootstrapping started...");

    let settings = if let Ok(custom_config_path) = env::var("CONFIG_PATH") {
        Settings::new_from_custom_config(custom_config_path)
    } else {
        Settings::new_default()
    }.expect("Config cannot be read as it's corrupted");
    debug!("Config has been built");

    let mut ledger = PullseLedger::new();

    let pullers = get_gatherers(&settings.gatherers);
    for puller in &pullers {
        let pulled_data = puller.gather();
        for entry in pulled_data {
            ledger.insert(entry);
        }
    }

    let consumers = get_exposers(&ledger, &settings.exposers);
    info!("Bootstrap completed");
    debug!("Ledger initial content {}", &ledger);

    info!("Starting runloop...");
    let (tx, rx) = channel();
    let pull_thread = thread::spawn(move || {
        loop {
            info!("Runloop: pull is in progress...");
            for puller in &pullers {
                let pulled_data = puller.gather();
                for entry in pulled_data {
                    tx.send(entry).unwrap(); // TODO: add proper error handling
                }
            }
            info!("Runloop: pull completed");
            thread::sleep(time::Duration::from_millis(settings.common.pull_timeout));
        }
    });

    let publish_thread = thread::spawn(move || while let Ok(entry) = rx.recv() {
        info!("Received metric {} = {}", entry.0, entry.1);
        ledger.insert(entry);
        for consumer in &consumers {
            consumer.consume(&ledger);
        }
        info!("Runloop: publish completed");
        debug!("Ledger content {}", &ledger);
    });

    info!("Runloop started");

    pull_thread.join().unwrap();
    publish_thread.join().unwrap();
}
