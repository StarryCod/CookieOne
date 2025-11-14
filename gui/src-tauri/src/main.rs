// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::env;
use std::sync::Mutex;

use once_cell::sync::{OnceCell, Lazy};
use platform_dirs::{AppDirs};
use tauri::{Manager, SystemTray};

// expose the config
mod config;

// include log
#[macro_use]
extern crate simple_log;
#[macro_use]
extern crate lazy_static;
mod log;

// include db
mod db;

// include tray
mod tray;

// include recorder
mod recorder;

// include speech-to-text (vosk)
mod vosk;

// include assistant_commands
mod assistant_commands;
use assistant_commands::AssistantCommand;

// include audio
mod audio;

// include tauri commands
mod tauri_commands;

// include events
mod events;

// some global data
pub(crate) static APP_DIR: Lazy<PathBuf> = Lazy::new(|| {env::current_dir().unwrap()});
pub(crate) static SOUND_DIR: Lazy<PathBuf> = Lazy::new(|| {APP_DIR.clone().join("sound")});
pub(crate) static APP_DIRS: OnceCell<AppDirs> = OnceCell::new();
pub(crate) static APP_CONFIG_DIR: OnceCell<PathBuf> = OnceCell::new();
pub(crate) static APP_LOG_DIR: OnceCell<PathBuf> = OnceCell::new();
pub(crate) static DB: OnceCell<Mutex<db::structs::Settings>> = OnceCell::new();
pub(crate) static COMMANDS: OnceCell<Vec<AssistantCommand>> = OnceCell::new();

fn main() {
    // Create the system tray
    let tray = tray::build_tray();

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            tray::handle_tray_event(app, &event);
        })
        .setup(|app| {
            // initialize directories
            config::init_dirs()?;

            // initialize logging
            log::init_logging()?;

            // log some base info
            info!("Starting Jarvis v{} ...", config::APP_VERSION.unwrap());
            info!("Config directory is: {}", APP_CONFIG_DIR.get().unwrap().display());
            info!("Log directory is: {}", APP_LOG_DIR.get().unwrap().display());

            // initialize database (settings)
            DB.set(Mutex::new(db::init_settings())).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // DB commands
            tauri_commands::db_read,
            tauri_commands::db_write,
            // Audio commands
            tauri_commands::pv_get_audio_devices,
            tauri_commands::pv_get_audio_device_name,
            // Listener commands
            tauri_commands::is_listening,
            tauri_commands::stop_listening,
            tauri_commands::start_listening,
            // System commands
            tauri_commands::get_current_ram_usage,
            tauri_commands::get_peak_ram_usage,
            tauri_commands::get_cpu_temp,
            tauri_commands::get_cpu_usage,
            // Voice commands
            tauri_commands::play_sound,
            // FS commands
            tauri_commands::get_app_path,
            // Onboarding commands
            tauri_commands::get_is_onboarding_completed,
            tauri_commands::set_onboarding_completed,
            tauri_commands::record_audio_sample,
            tauri_commands::train_wakeword,
            // Etc commands
            tauri_commands::get_app_version,
            tauri_commands::get_author_name,
            tauri_commands::get_repository_link,
            tauri_commands::get_tg_official_link,
            tauri_commands::get_feedback_link,
            tauri_commands::get_log_file_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
