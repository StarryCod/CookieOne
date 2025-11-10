use super::SpeechToText;
use crate::APP_DIR;
use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use vosk::{Model, Recognizer};

pub struct VoskEngine {
    recognizer: Mutex<Recognizer>,
}

impl VoskEngine {
    pub fn new<S: AsRef<str>>(model_path: S) -> Result<Self> {
        let path = APP_DIR.join(model_path.as_ref());
        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid model path"))?;
        let model =
            Model::new(path_str).ok_or_else(|| anyhow::anyhow!("Failed to load Vosk model"))?;
        let mut recognizer = Recognizer::new(&model, 16000.0)
            .ok_or_else(|| anyhow::anyhow!("Failed to create recognizer"))?;
        recognizer.set_max_alternatives(1);
        recognizer.set_words(true);

        Ok(Self {
            recognizer: Mutex::new(recognizer),
        })
    }
}

impl SpeechToText for VoskEngine {
    fn transcribe(&self, pcm: &[i16]) -> Result<Option<String>> {
        let mut rec = self.recognizer.lock();

        if let Ok(state) = rec.accept_waveform(pcm) {
            if matches!(state, vosk::DecodingState::Finalized) {
                let result = rec.result();
                let text = match result {
                    vosk::CompleteResult::Single(single) => single.text.trim(),
                    vosk::CompleteResult::Multiple(multi) => multi
                        .alternatives
                        .first()
                        .map(|alt| alt.text.trim())
                        .unwrap_or(""),
                };

                if !text.is_empty() {
                    return Ok(Some(text.to_string()));
                }
            }
        }

        Ok(None)
    }
}
