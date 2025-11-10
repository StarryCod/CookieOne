use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use vosk::{Model, Recognizer};

#[derive(Debug, Deserialize)]
struct WakewordConfig {
    model_path: PathBuf,
    keyphrase: String,
    #[serde(default = "default_threshold")]
    threshold: f32,
}

fn default_threshold() -> f32 {
    0.45
}

pub struct WakeWordDetector {
    phrase: String,
    threshold: f32,
    recognizer: Recognizer,
}

impl WakeWordDetector {
    pub fn from_config_file<P: AsRef<Path>>(path: P, sample_rate: f32) -> Result<Self> {
        let mut file = File::open(path.as_ref()).context("Unable to open wakeword config")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Unable to read wakeword config")?;
        let config: WakewordConfig =
            serde_json::from_str(&contents).context("Invalid wakeword config JSON")?;

        let model_path = config
            .model_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid model path"))?;
        let model =
            Model::new(model_path).ok_or_else(|| anyhow::anyhow!("Failed to load Vosk model"))?;

        let grammar = [config.keyphrase.as_str()];
        let recognizer = Recognizer::new_with_grammar(&model, sample_rate, &grammar)
            .ok_or_else(|| anyhow::anyhow!("Failed to build Vosk recognizer"))?;

        Ok(Self {
            phrase: config.keyphrase,
            threshold: config.threshold,
            recognizer,
        })
    }

    pub fn process(&mut self, pcm: &[i16]) -> bool {
        match self.recognizer.accept_waveform(pcm) {
            Ok(state) => {
                if matches!(state, vosk::DecodingState::Finalized) {
                    let result = self.recognizer.result();
                    let text = match result {
                        vosk::CompleteResult::Single(s) => s.text,
                        vosk::CompleteResult::Multiple(m) => {
                            m.alternatives.first().map(|alt| alt.text).unwrap_or("")
                        }
                    };
                    if text.trim().to_lowercase() == self.phrase {
                        return true;
                    }
                }
            }
            Err(_) => {}
        }
        false
    }

    pub fn phrase(&self) -> &str {
        &self.phrase
    }

    pub fn threshold(&self) -> f32 {
        self.threshold
    }
}
