pub mod structs;

use std::fs;
use std::env;
use std::path::PathBuf;

use platform_dirs::AppDirs;
use rustpotter::{RustpotterConfig, WavFmt, DetectorConfig, FiltersConfig, ScoreMode, GainNormalizationConfig, BandPassConfig};

use crate::{APP_DIRS, APP_CONFIG_DIR, APP_LOG_DIR};

pub use structs::{WakeWordEngine, SpeechToTextEngine};

pub fn init_dirs() -> Result<(), String> {
    if APP_DIRS.get().is_some() {
        return Ok(());
    }

    APP_DIRS.set(AppDirs::new(Some(BUNDLE_IDENTIFIER), false).unwrap()).unwrap();

    let mut config_dir = PathBuf::from(&APP_DIRS.get().unwrap().config_dir);
    let mut log_dir = PathBuf::from(&APP_DIRS.get().unwrap().config_dir);

    if !config_dir.exists() {
        if fs::create_dir_all(&config_dir).is_err() {
            config_dir = env::current_dir().expect("Cannot infer the config directory");
            fs::create_dir_all(&config_dir).expect("Cannot create config directory, access denied?");
        }
    }

    if !log_dir.exists() {
        if fs::create_dir_all(&log_dir).is_err() {
            log_dir = env::current_dir().expect("Cannot infer the log directory");
            fs::create_dir_all(&log_dir).expect("Cannot create log directory, access denied?");
        }
    }

    APP_CONFIG_DIR.set(config_dir).unwrap();
    APP_LOG_DIR.set(log_dir).unwrap();

    Ok(())
}

pub const DEFAULT_WAKE_WORD_ENGINE: WakeWordEngine = WakeWordEngine::Rustpotter;
pub const DEFAULT_SPEECH_TO_TEXT_ENGINE: SpeechToTextEngine = SpeechToTextEngine::Vosk;

pub const BUNDLE_IDENTIFIER: &str = "com.priler.cookie";
pub const DB_FILE_NAME: &str = "app.db";
pub const LOG_FILE_NAME: &str = "log.txt";
pub const APP_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const AUTHOR_NAME: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
pub const REPOSITORY_LINK: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");
pub const TG_OFFICIAL_LINK: Option<&str> = Some("https://t.me/cookie_assistant");
pub const FEEDBACK_LINK: Option<&str> = Some("https://t.me/cookie_feedback_bot");

pub const TRAY_ICON: &str = "32x32.png";
pub const TRAY_TOOLTIP: &str = "Cookie Voice Assistant";

pub const RUSPOTTER_MIN_SCORE: f32 = 0.62;

pub const RUSTPOTTER_DEFAULT_CONFIG: RustpotterConfig = RustpotterConfig {
    fmt: WavFmt {
        sample_rate: 16000,
        sample_format: rustpotter::SampleFormat::I16,
        channels: 1,
    },
    detector: DetectorConfig {
        avg_threshold: 0.0,
        threshold: 0.5,
        min_scores: 15,
        score_mode: ScoreMode::Average,
        comparator_band_size: 5,
        comparator_ref: 0.22,
    },
    filters: FiltersConfig {
        gain_normalizer: GainNormalizationConfig {
            enabled: true,
            gain_ref: None,
            min_gain: 0.7,
            max_gain: 1.0,
        },
        band_pass: BandPassConfig {
            enabled: true,
            low_cutoff: 80.0,
            high_cutoff: 400.0,
        },
    },
};

pub const KEYWORDS_PATH: &str = "keywords/";
pub const VOSK_FETCH_PHRASE: &str = "cookie";
pub const VOSK_MODEL_PATH: &str = "models/vosk-model-small-ru-0.22";
pub const VOSK_MIN_RATIO: f64 = 70.0;

pub const CMD_RATIO_THRESHOLD: f64 = 65f64;
pub const CMS_WAIT_DELAY: std::time::Duration = std::time::Duration::from_secs(15);

pub const ASSISTANT_GREET_PHRASES: [&str; 3] = ["greet1", "greet2", "greet3"];
pub const ASSISTANT_PHRASES_TBR: [&str; 17] = [
    "cookie",
    "куки",
    "сэр",
    "слушаю",
    "всегда к услугам",
    "произнеси",
    "ответь",
    "покажи",
    "скажи",
    "давай",
    "да",
    "к вашим услугам",
    "всегда к вашим услугам",
    "запрос выполнен",
    "выполнен",
    "есть",
    "загружаю",
];
