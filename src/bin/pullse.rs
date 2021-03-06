use pullse::app::App;
use pullse::settings::Settings;
use simple_logger::SimpleLogger;
use std::env;

fn main() {
    if let Err(error) = SimpleLogger::new().with_utc_timestamps().env().init() {
        println!(
            "Unable to properly setup logger, logs will be incomplete: {}",
            error
        );
    }

    let settings = if let Ok(custom_config_path) = env::var("CONFIG_PATH") {
        Settings::new_from_custom_config(custom_config_path)
    } else {
        Settings::new_from_default_config()
    }
    .expect("Config cannot be read as it's corrupted");

    let app = App::new(settings);
    app.run();
}
