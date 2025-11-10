mod gemini_audio;
mod vosk_engine;

use anyhow::Result;
use once_cell::sync::OnceCell;

use crate::config::{self, SttBackendConfig};

static ENGINE: OnceCell<Box<dyn SpeechToText + Send + Sync>> = OnceCell::new();

pub trait SpeechToText {
    fn transcribe(&self, pcm: &[i16]) -> Result<Option<String>>;
}

pub fn init() -> Result<()> {
    let cfg = config::config();
    match &cfg.stt_backend {
        SttBackendConfig::Vosk { model_path } => {
            let engine = vosk_engine::VoskEngine::new(model_path)?;
            ENGINE
                .set(Box::new(engine))
                .map_err(|_| anyhow::anyhow!("STT engine already initialized"))?;
            info!("Vosk STT engine initialized");
        }
        SttBackendConfig::GeminiAudio => {
            let api_key = cfg
                .gemini_api_key
                .clone()
                .ok_or_else(|| anyhow::anyhow!("Gemini API key not provided"))?;
            let engine = gemini_audio::GeminiAudioEngine::new(api_key)?;
            ENGINE
                .set(Box::new(engine))
                .map_err(|_| anyhow::anyhow!("STT engine already initialized"))?;
            info!("Gemini Audio STT engine initialized");
        }
    }

    Ok(())
}

pub fn recognize(pcm: &[i16], _partial: bool) -> Option<String> {
    if let Some(engine) = ENGINE.get() {
        match engine.transcribe(pcm) {
            Ok(result) => result,
            Err(err) => {
                error!("STT error: {}", err);
                None
            }
        }
    } else {
        None
    }
}
