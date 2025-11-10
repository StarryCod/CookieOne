use crate::{APP_CONFIG_DIR, APP_DIR, APP_DIRS};
use once_cell::sync::Lazy;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const DEFAULT_CONFIG_FILE: &str = "config.json";
pub const GEMINI_ENDPOINT: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-pro-audio:generateContent";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SttBackendConfig {
    Vosk { model_path: String },
    GeminiAudio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub wake_word_threshold: f32,
    pub wake_word_path: String,
    pub stt_backend: SttBackendConfig,
    pub gemini_api_key: Option<String>,
    pub jarvis_phrases: String,
    pub commands_path: String,
    pub listening_device: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            wake_word_threshold: 0.45,
            wake_word_path: "assets/wakeword/cookie.mww".into(),
            stt_backend: SttBackendConfig::Vosk {
                model_path: "assets/stt/vosk-model-small-ru-0.22".into(),
            },
            gemini_api_key: None,
            jarvis_phrases: "assets/phrases/jarvis_style.json".into(),
            commands_path: "commands/commands.json".into(),
            listening_device: 0,
        }
    }
}

static CONFIG: Lazy<AppConfig> = Lazy::new(|| load_config().unwrap_or_default());

pub fn init_dirs() -> anyhow::Result<()> {
    if APP_DIRS.get().is_some() {
        return Ok(());
    }

    let dirs = AppDirs::new(Some("cookie"), false).unwrap();
    let config_dir = dirs.config_dir.clone();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    APP_DIRS.set(dirs).ok();
    APP_CONFIG_DIR.set(config_dir).ok();

    Ok(())
}

pub fn config() -> &'static AppConfig {
    &CONFIG
}

pub fn update_gemini_key(key: String) -> anyhow::Result<()> {
    let mut cfg = CONFIG.clone();
    cfg.gemini_api_key = Some(key);
    save_config(&cfg)
}

fn config_path() -> PathBuf {
    APP_DIR.join(DEFAULT_CONFIG_FILE)
}

fn load_config() -> anyhow::Result<AppConfig> {
    let path = config_path();
    if !path.exists() {
        let cfg = AppConfig::default();
        save_config(&cfg)?;
        return Ok(cfg);
    }

    let contents = fs::read_to_string(&path)?;
    let cfg: AppConfig = serde_json::from_str(&contents)?;
    Ok(cfg)
}

fn save_config(cfg: &AppConfig) -> anyhow::Result<()> {
    let contents = serde_json::to_string_pretty(cfg)?;
    fs::write(config_path(), contents)?;
    Ok(())
}
