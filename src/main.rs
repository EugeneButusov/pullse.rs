use std::{thread, time};
use measure::{DataPuller, Registry};

const PULL_SLEEP_MS: u64 = 5 * 1000;
const PUBLISH_SLEEP_MS: u64 = 3 * 1000;

fn main() {
    println!("Bootstrapping started...");
    let mut registry = Registry::new();

    let pullers = get_pullers();
    for puller in &pullers {
        let pulled_data = puller.pull_data();
        registry.insert(&pulled_data);
    }
    println!("Bootstrapping completed!");

    println!("Runloop initiated");

    let pull_thread = thread::spawn(|| {
        loop {
            // TODO: perform pull
            for puller in &pullers {
                let pulled_data = puller.pull_data();
                registry.insert(&pulled_data);
            }
            thread::sleep(time::Duration::from_millis(PULL_SLEEP_MS));
        }
    });

    let publish_thread = thread::spawn(|| {
        // TODO: perform real publish instead of println
        println!("{:?}", registry);
        thread::sleep(time::Duration::from_millis(PUBLISH_SLEEP_MS));
    });

    pull_thread.join().unwrap();
    publish_thread.join().unwrap();
}

fn get_pullers() -> Vec<DataPuller> {
    let mut result = Vec::new();

    result.push(DataPuller::new());

    result
}
