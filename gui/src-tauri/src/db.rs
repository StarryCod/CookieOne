pub mod structs;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::{config, APP_CONFIG_DIR};

pub fn init_settings() -> Mutex<structs::Settings> {
    let mut settings = None;
    let db_file_path = get_db_file_path();

    info!("Loading settings db file located at: {}", db_file_path.display());

    if db_file_path.exists() {
        if let Ok(file) = File::open(&db_file_path) {
            let reader = BufReader::new(file);
            if let Ok(parsed) = serde_json::from_reader(reader) {
                info!("Settings loaded.");
                settings = Some(parsed);
            }
        }
    }

    if settings.is_none() {
        warn!("No settings file found or there was an error parsing it. Creating default struct.");
        settings = Some(structs::Settings::default());
    }

    Mutex::new(settings.unwrap())
}

pub fn save_settings(settings: &structs::Settings) -> Result<(), std::io::Error> {
    let db_file_path = get_db_file_path();

    std::fs::write(
        &db_file_path,
        serde_json::to_string_pretty(settings).unwrap(),
    )?;

    info!("Settings saved at {}", db_file_path.display());
    Ok(())
}

fn get_db_file_path() -> PathBuf {
    APP_CONFIG_DIR
        .get()
        .expect("Config dir not initialised")
        .join(config::DB_FILE_NAME)
}
