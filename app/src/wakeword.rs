use anyhow::{anyhow, Context, Result};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use vosk::{Model, Recognizer};

use crate::{config, APP_DIR};

#[derive(Debug, Deserialize)]
struct WakewordConfig {
    model_path: String,
    keyphrase: String,
    #[serde(default = "default_threshold")]
    threshold: f32,
}

fn default_threshold() -> f32 {
    0.45
}

struct WakeWordDetector {
    phrase: String,
    threshold: f32,
    recognizer: Recognizer,
}

impl WakeWordDetector {
    fn from_config_file(config_path: &std::path::Path, sample_rate: f32) -> Result<Self> {
        // Загружаем конфигурацию wake word
        let mut file = File::open(config_path).context("Unable to open wakeword config")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Unable to read wakeword config")?;
        let config: WakewordConfig = serde_json::from_str(&contents)
            .context("Invalid wakeword config JSON")?;

        // Загружаем Vosk модель
        let model_path = APP_DIR.join(&config.model_path);
        let model_path_str = model_path
            .to_str()
            .ok_or_else(|| anyhow!("Invalid model path"))?;
        let model = Model::new(model_path_str).ok_or_else(|| anyhow!("Failed to load Vosk model"))?;

        // Создаем recognizer с grammar (только wake word)
        let grammar = [config.keyphrase.as_str()];
        let recognizer = Recognizer::new_with_grammar(&model, sample_rate, &grammar)
            .ok_or_else(|| anyhow!("Failed to build Vosk recognizer"))?;

        Ok(Self {
            phrase: config.keyphrase.to_lowercase(),
            threshold: config.threshold,
            recognizer,
        })
    }

    fn process(&mut self, pcm: &[i16]) -> bool {
        match self.recognizer.accept_waveform(pcm) {
            Ok(state) => {
                if matches!(state, vosk::DecodingState::Finalized) {
                    let result = self.recognizer.result();
                    let text = match result {
                        vosk::CompleteResult::Single(s) => s.text.trim().to_lowercase(),
                        vosk::CompleteResult::Multiple(m) => m
                            .alternatives
                            .first()
                            .map(|alt| alt.text.trim().to_lowercase())
                            .unwrap_or_default(),
                    };

                    if text == self.phrase {
                        return true;
                    }
                }
            }
            Err(_) => {}
        }
        false
    }

    fn phrase(&self) -> &str {
        &self.phrase
    }

    fn threshold(&self) -> f32 {
        self.threshold
    }
}

static DETECTOR: OnceCell<Mutex<WakeWordDetector>> = OnceCell::new();

pub fn init() -> Result<()> {
    info!("Initializing wake word detector (Vosk-based)...");

    let cfg = config::config();
    let path = APP_DIR.join(&cfg.wake_word_path);

    let detector = WakeWordDetector::from_config_file(&path, 16000.0)?;
    info!(
        "Wake word detector initialized: phrase='{}', threshold={}",
        detector.phrase(),
        detector.threshold()
    );

    DETECTOR
        .set(Mutex::new(detector))
        .map_err(|_| anyhow!("Detector already initialized"))?;

    Ok(())
}

pub fn detect(pcm: &[i16]) -> bool {
    if let Some(detector) = DETECTOR.get() {
        detector.lock().process(pcm)
    } else {
        false
    }
}
