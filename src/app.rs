use crate::exposing::get_exposers;
use crate::gathering::get_gatherers;
use crate::ledger::PullseLedger;
use crate::settings::Settings;
use crate::{exposing, gathering};
use log::{debug, info};
use std::sync::mpsc::channel;
use std::{thread, time};

pub struct App {
    settings: Box<Settings>,
    ledger: Box<PullseLedger>,
    gatherers: Vec<Box<dyn gathering::common::PullseGatherer + Send + Sync>>,
    exposers: Vec<Box<dyn exposing::common::PullseExposer + Send + Sync>>,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        let settings = Box::new(settings);

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

        App {
            settings,
            ledger,
            exposers,
            gatherers,
        }
    }

    pub fn run(self) {
        info!("Starting runloop...");
        let (sender_handle, receiver_handle) = channel();

        let gatherers = self.gatherers;
        let pull_timeout = self.settings.common.pull_timeout;
        let gathering_thread = thread::spawn(move || loop {
            info!("Runloop: gathering cycle is in progress...");
            for gatherer in &gatherers {
                let gathered_data = gatherer.gather();
                for entry in gathered_data {
                    sender_handle.send(entry)
                        .expect("Gathered data cannot be sent to exposers");
                }
            }
            info!("Runloop: gathering cycle completed");
            thread::sleep(time::Duration::from_millis(pull_timeout));
        });

        let mut ledger = self.ledger;
        let exposers = self.exposers;
        let exposing_thread = thread::spawn(move || {
            while let Ok(entry) = receiver_handle.recv() {
                info!("Received metric {} = {}", entry.0, entry.1);
                ledger.insert(entry);
                for exposer in &exposers {
                    exposer.consume(&ledger);
                }
                info!("Runloop: exposing cycle completed");
                debug!("Ledger content {}", &ledger);
            }
        });

        info!("Runloop started");

        gathering_thread.join().unwrap();
        exposing_thread.join().unwrap();
    }
}
