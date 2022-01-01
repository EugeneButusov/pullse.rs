use std::{env, thread, time};
use std::sync::mpsc::channel;
use log::{debug, info};
use simple_logger::SimpleLogger;
use crate::{exposing, gathering};
use crate::exposing::get_exposers;
use crate::gathering::get_gatherers;
use crate::ledger::PullseLedger;
use crate::settings::Settings;

struct App {
    settings: Box<Settings>,
    ledger: Box<PullseLedger>,
    gatherers: Vec<Box<dyn gathering::common::PullseGatherer + Send + Sync>>,
    exposers: Vec<Box<dyn exposing::common::PullseExposer + Send + Sync>>,
}

impl App {
    fn new() -> App {
        SimpleLogger::new().with_utc_timestamps().env().init().unwrap();
        info!("Bootstrapping started...");

        let settings = if let Ok(custom_config_path) = env::var("CONFIG_PATH") {
            Settings::new_from_custom_config(custom_config_path)
        } else {
            Settings::new_default()
        }.expect("Config cannot be read as it's corrupted");
        debug!("Config has been built");

        let mut ledger = Box::new(PullseLedger::new());

        let gatherers = get_gatherers(&settings.gatherers);
        for gatherer in &gatherers {
            let pulled_data = gatherer.gather();
            for entry in pulled_data {
                ledger.insert(entry);
            }
        }

        let exposers = get_exposers(&ledger, &settings.exposers);
        info!("Bootstrap completed");
        debug!("Ledger initial content {}", &ledger);

        App{ settings: Box::new(settings), ledger, exposers, gatherers }
    }

    fn run(&mut self) {
        info!("Starting runloop...");
        let (tx, rx) = channel();
        let gatherers = &self.gatherers;
        let pull_thread = thread::spawn(move || loop {
            info!("Runloop: pull is in progress...");
            for gatherer in gatherers {
                let gathered_data = gatherer.gather();
                for entry in gathered_data {
                    tx.send(entry).unwrap(); // TODO: add proper error handling
                }
            }
            info!("Runloop: pull completed");
            thread::sleep(time::Duration::from_millis(self.settings.common.pull_timeout));
        });

        let mut ledger = &self.ledger;
        let exposers = &self.exposers;
        let publish_thread = thread::spawn(move || while let Ok(entry) = rx.recv() {
            info!("Received metric {} = {}", entry.0, entry.1);
            ledger.insert(entry);
            for exposer in exposers {
                exposer.consume(ledger);
            }
            info!("Runloop: publish completed");
            debug!("Ledger content {}", ledger);
        });

        info!("Runloop started");

        pull_thread.join().unwrap();
        publish_thread.join().unwrap();
    }
}
