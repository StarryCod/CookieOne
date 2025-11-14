// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::env;
use std::sync::Mutex;

use once_cell::sync::{OnceCell, Lazy};
use platform_dirs::{AppDirs};

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

fn main() -> Result<(), String> {
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
}