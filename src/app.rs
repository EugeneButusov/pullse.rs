use std::{thread, time};
use std::sync::{Mutex};
use std::sync::mpsc::channel;
use log::{debug, error, info};
use crate::{exposing, gathering};
use crate::exposing::get_exposers;
use crate::gathering::get_gatherers;
use crate::ledger::PullseLedger;
use crate::settings::Settings;

pub struct App {
    settings: Box<Settings>,
    ledger: Box<PullseLedger>,
    gatherers: Vec<Box<dyn gathering::common::PullseGatherer + Send + Sync>>,
    exposers: Vec<Box<dyn exposing::common::PullseExposer + Send + Sync>>,
}

impl App {
    pub fn new(settings: Settings) -> App {
        info!("Bootstrapping started...");

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

    pub fn run(&'static mut self) {
        info!("Starting runloop...");
        let (tx, rx) = channel();

        let gatherers = &self.gatherers;
        let pull_timeout = self.settings.common.pull_timeout;
        let pull_thread = thread::spawn(move || loop {
            info!("Runloop: pull is in progress...");
            for gatherer in gatherers {
                let gathered_data = gatherer.gather();
                for entry in gathered_data {
                    tx.send(entry).unwrap(); // TODO: add proper error handling
                }
            }
            info!("Runloop: pull completed");
            thread::sleep(time::Duration::from_millis(pull_timeout));
        });

        let ledger_mutex = Mutex::new(&self.ledger);
        let exposers = &self.exposers;
        let publish_thread = thread::spawn(move || while let Ok(entry) = rx.recv() {
            info!("Received metric {} = {}", entry.0, entry.1);
            if let Ok(mut ledger) = ledger_mutex.lock() {
                ledger.insert(entry);
                for exposer in exposers {
                    exposer.consume(&ledger);
                }
                info!("Runloop: publish completed");
                debug!("Ledger content {}", &ledger);
            } else {
                error!("Cannot lock ledger");
            }
        });

        info!("Runloop started");

        pull_thread.join().unwrap();
        publish_thread.join().unwrap();
    }
}
