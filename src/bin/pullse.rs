use std::{thread, time};
use std::sync::mpsc::channel;
use measure::{Registry};



const PULL_SLEEP_MS: u64 = 5 * 1000;

fn main() {
    println!("Bootstrapping started...");
    let mut registry = Registry::new();

    let pullers = get_pullers();
    for puller in &pullers {
        let pulled_data = puller.pull_data();
        for entry in pulled_data {
            registry.insert(entry);
        }
    }
    println!("Bootstrapping completed!");

    println!("Runloop initiated");

    let (tx, rx) = channel();
    let pull_thread = thread::spawn(move || {
        loop {
            // TODO: perform pull
            for puller in &pullers {
                let pulled_data = puller.pull_data();
                for entry in pulled_data {
                    tx.send(entry).unwrap(); // TODO: add proper error handling
                }
            }
            thread::sleep(time::Duration::from_millis(PULL_SLEEP_MS));
        }
    });

    let publish_thread = thread::spawn(move || while let Ok(entry) = rx.recv() {
        registry.insert(entry);
        // TODO: perform real publish instead of println
        println!("{:?}", &registry);
    });

    pull_thread.join().unwrap();
    publish_thread.join().unwrap();
}
