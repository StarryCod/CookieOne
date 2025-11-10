use once_cell::sync::{Lazy, OnceCell};
use platform_dirs::AppDirs;
use std::env;
use std::path::PathBuf;

#[macro_use]
extern crate simple_log;

mod audio;
mod commands;
mod config;
mod persona;
mod recorder;
mod stt;
mod wakeword;

static APP_DIR: Lazy<PathBuf> = Lazy::new(|| env::current_dir().unwrap());
static APP_DIRS: OnceCell<AppDirs> = OnceCell::new();
static APP_CONFIG_DIR: OnceCell<PathBuf> = OnceCell::new();

fn main() -> anyhow::Result<()> {
    // Initialize directories
    config::init_dirs()?;

    // Initialize logging
    simple_log::quick!("info");
    info!(
        "Starting Cookie Voice Assistant v{}",
        env!("CARGO_PKG_VERSION")
    );

    // Initialize persona
    persona::init()?;

    // Initialize audio recorder
    recorder::init()?;

    // Initialize wake word detector
    wakeword::init()?;

    // Initialize STT engine
    stt::init()?;

    // Initialize audio playback
    audio::init()?;

    // Load commands
    commands::init()?;

    // Start main loop
    run_main_loop()?;

    Ok(())
}

fn run_main_loop() -> anyhow::Result<()> {
    info!("Starting main loop...");

    let mut audio_buffer = vec![0i16; 512];
    recorder::start()?;

    loop {
        // Read audio from microphone
        recorder::read(&mut audio_buffer)?;

        // Check for wake word
        if wakeword::detect(&audio_buffer) {
            info!("Wake word detected!");
            audio::play(persona::get_wake_phrase())?;
            audio::play(persona::get_ack_phrase())?;

            // Listen for command
            let timeout = std::time::Duration::from_secs(5);
            let start = std::time::Instant::now();

            loop {
                if start.elapsed() > timeout {
                    info!("Listening timeout");
                    break;
                }

                recorder::read(&mut audio_buffer)?;

                if let Some(text) = stt::recognize(&audio_buffer, false) {
                    info!("Recognized: {}", text);

                    // Execute command
                    if let Some(cmd) = commands::match_command(&text) {
                        info!("Executing command: {:?}", cmd);
                        audio::play(persona::get_processing_phrase())?;
                        match commands::execute(&cmd) {
                            Ok(_) => {
                                audio::play(persona::get_done_phrase())?;
                            }
                            Err(err) => {
                                error!("Command execution failed: {}", err);
                                audio::play(persona::get_error_phrase())?;
                            }
                        }
                    } else {
                        audio::play(persona::get_error_phrase())?;
                    }
                    break;
                }
            }
        }
    }
}
