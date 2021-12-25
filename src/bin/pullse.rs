use std::{thread, time};
use std::sync::mpsc::channel;
use pullse::ledger::{PullseLedger};
use pullse::gathering::get_gatherers;
use pullse::exposing::get_exposers;
use pullse::settings::Settings;


const PULL_SLEEP_MS: u64 = 5 * 1000;

fn main() {
    let settings = Settings::new(String::from(""));
    println!("{:?}", settings);

    println!("Bootstrapping started...");
    let mut ledger = PullseLedger::new();

    let pullers = get_gatherers();
    for puller in &pullers {
        let pulled_data = puller.gather();
        for entry in pulled_data {
            ledger.insert(entry);
        }
    }

    let consumers = get_exposers(&ledger);
    println!("Bootstrapping completed!");

    println!("Runloop initiated");

    let (tx, rx) = channel();
    let pull_thread = thread::spawn(move || {
        loop {
            // TODO: perform pull
            for puller in &pullers {
                let pulled_data = puller.gather();
                for entry in pulled_data {
                    tx.send(entry).unwrap(); // TODO: add proper error handling
                }
            }
            thread::sleep(time::Duration::from_millis(PULL_SLEEP_MS));
        }
    });

    let publish_thread = thread::spawn(move || while let Ok(entry) = rx.recv() {
        ledger.insert(entry);
        for consumer in &consumers {
            consumer.consume(&ledger);
        }
    });

    pull_thread.join().unwrap();
    publish_thread.join().unwrap();
}
