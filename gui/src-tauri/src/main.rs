// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::{Lazy, OnceCell};
use platform_dirs::{AppDirs};

// expose the config
mod config;

// include log
#[macro_use]
extern crate simple_log;
mod log;

// include db
mod db;

// include recorder
mod recorder;

// include speech-to-text
mod stt;

// include commands
mod commands;
use commands::AssistantCommand;

// include audio
mod audio;

// include listener
mod listener;

// include events
mod events;

// include tauri commands
mod tauri_commands;

// some global data
static APP_DIR: Lazy<PathBuf> = Lazy::new(|| {env::current_dir().unwrap()});
static SOUND_DIR: Lazy<PathBuf> = Lazy::new(|| {APP_DIR.clone().join("sound")});
static APP_DIRS: OnceCell<AppDirs> = OnceCell::new();
static APP_CONFIG_DIR: OnceCell<PathBuf> = OnceCell::new();
static APP_LOG_DIR: OnceCell<PathBuf> = OnceCell::new();
static DB: OnceCell<Mutex<db::structs::Settings>> = OnceCell::new();
static COMMANDS: OnceCell<Vec<AssistantCommand>> = OnceCell::new();

// aliases
use commands as assistant_commands;
use stt as vosk;

fn main() {
    // initialize directories
    config::init_dirs().expect("Failed to initialize directories");

    // initialize logging
    log::init_logging().expect("Failed to initialize logging");

    // log some base info
    info!("Starting Jarvis v{} ...", config::APP_VERSION.unwrap());
    info!("Config directory is: {}", APP_CONFIG_DIR.get().unwrap().display());
    info!("Log directory is: {}", APP_LOG_DIR.get().unwrap().display());

    // initialize database (settings)
    DB.set(Mutex::new(db::init_settings())).expect("Failed to initialize DB");

    // initialize recorder
    if let Err(e) = recorder::init() {
        error!("Failed to initialize recorder: {}", e);
    }

    // initialize speech-to-text
    if let Err(e) = stt::init() {
        error!("Failed to initialize STT: {}", e);
    }

    // initialize commands
    info!("Initializing commands.");
    match commands::parse_commands() {
        Ok(cmds) => {
            info!("Commands initialized.\nOverall commands parsed: {}\nParsed commands: {:?}", 
                cmds.len(), commands::list(&cmds));
            COMMANDS.set(cmds).expect("Failed to set commands");
        },
        Err(e) => {
            warn!("Failed to parse commands: {}", e);
            COMMANDS.set(vec![]).expect("Failed to set empty commands");
        }
    }

    // initialize audio
    if let Err(e) = audio::init() {
        error!("Failed to initialize audio: {}", e);
    }

    // Build and run Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // DB commands
            tauri_commands::db_read,
            tauri_commands::db_write,
            // Audio commands
            tauri_commands::pv_get_audio_devices,
            tauri_commands::pv_get_audio_device_name,
            // Listener commands
            tauri_commands::start_listening,
            tauri_commands::stop_listening,
            tauri_commands::is_listening,
            // Voice commands
            tauri_commands::get_voice_directory,
            tauri_commands::get_all_voices,
            // System commands
            tauri_commands::get_sys_platform,
            tauri_commands::get_sys_arch,
            tauri_commands::get_sys_version,
            // FS commands
            tauri_commands::show_in_explorer,
            tauri_commands::open_url,
            // Etc commands
            tauri_commands::get_app_version,
            tauri_commands::get_tg_official_link,
            tauri_commands::get_feedback_link,
            tauri_commands::get_repository_link,
            tauri_commands::get_log_file_path,
            // Commands
            tauri_commands::get_commands_list,
            tauri_commands::execute_command,
            tauri_commands::get_jarvis_phrase,
            // Config commands
            tauri_commands::get_config,
            tauri_commands::save_gemini_api_key,
            tauri_commands::set_listening_device,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
