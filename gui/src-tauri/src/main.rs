// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::OnceCell;
use platform_dirs::AppDirs;

// expose the config
mod config;

// include log
#[macro_use]
extern crate simple_log;
mod log;

// include db
mod db;

// include tray
mod tray;

// include events
mod events;

// include recorder
mod recorder;

// include vosk
mod vosk;

// include assistant_commands
mod assistant_commands;

// include audio
mod audio;

// include tauri commands
mod tauri_commands;
use tauri_commands::*;

// some global data
pub(crate) static APP_DIRS: OnceCell<AppDirs> = OnceCell::new();
pub(crate) static APP_CONFIG_DIR: OnceCell<PathBuf> = OnceCell::new();
pub(crate) static APP_LOG_DIR: OnceCell<PathBuf> = OnceCell::new();
pub(crate) static DB: OnceCell<Mutex<db::structs::Settings>> = OnceCell::new();
pub(crate) static COMMANDS: OnceCell<Vec<assistant_commands::CommandConfig>> = OnceCell::new();

fn main() {
    // initialize directories
    config::init_dirs().expect("Failed to initialize directories");

    // initialize logging
    log::init_logging().expect("Failed to initialize logging");

    // log some base info
    info!(
        "Starting Cookie v{} ...",
        config::APP_VERSION.unwrap_or("unknown")
    );
    info!(
        "Config directory is: {}",
        APP_CONFIG_DIR.get().unwrap().display()
    );
    info!("Log directory is: {}", APP_LOG_DIR.get().unwrap().display());

    // initialize database (settings)
    DB.set(db::init_settings()).expect("Failed to init DB");

    // initialize commands list (empty stub for now)
    COMMANDS.set(Vec::new()).expect("Failed to init COMMANDS");

    if let Err(err) = recorder::init() {
        warn!("Failed to init recorder: {}", err);
    }

    if let Err(err) = vosk::init() {
        warn!("Failed to init Vosk: {}", err);
    }

    if let Err(err) = audio::init() {
        warn!("Failed to init audio: {}", err);
    }

    // initialize events
    events::init();

    // initialize tray
    tray::init();

    info!("Starting Tauri application...");

    // run Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_listening,
            start_listening,
            stop_listening,
            db_read,
            db_write,
            get_is_onboarding_completed,
            set_onboarding_completed,
            train_wakeword,
            record_audio_sample,
            pv_get_audio_devices,
            pv_get_audio_device_name,
            get_app_version,
            get_author_name,
            get_tg_official_link,
            get_feedback_link,
            get_repository_link,
            get_log_file_path,
            show_in_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}