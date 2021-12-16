use measure::{DataPuller, Registry};

fn main() {
    println!("Bootstrapping started...");
    let mut registry = Registry::new();

    let pullers = get_pullers();
    for puller in pullers {
        let pulled_data = puller.pull_data();
        registry.insert(&pulled_data);
    }
    println!("Bootstrapping completed!");

    println!("{:?}", registry);
    println!("Runloop initiated");

    // TODO: implement runloop
}

fn get_pullers() -> Vec<DataPuller> {
    let mut result = Vec::new();

    result.push(DataPuller::new());

    result
}
