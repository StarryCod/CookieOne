pub mod vosk;
pub mod gemini_audio;

use anyhow::Result;
use async_trait::async_trait;

pub enum SttBackend {
    Vosk(vosk::VoskStt),
    Gemini(gemini_audio::GeminiStt),
}

impl SttBackend {
    /// Создает STT движок в зависимости от конфигурации
    pub fn from_config(config: &crate::config::SttBackendConfig, gemini_api_key: Option<String>) -> Result<Self> {
        match config {
            crate::config::SttBackendConfig::Vosk { model_path } => {
                let engine = vosk::VoskStt::new(model_path)?;
                Ok(SttBackend::Vosk(engine))
            }
            crate::config::SttBackendConfig::Gemini => {
                let engine = gemini_audio::GeminiStt::new(gemini_api_key)?;
                Ok(SttBackend::Gemini(engine))
            }
        }
    }
}

#[async_trait]
pub trait SpeechToText {
    async fn transcribe(&mut self, pcm: &[i16]) -> Result<String>;
}

#[async_trait]
impl SpeechToText for SttBackend {
    async fn transcribe(&mut self, pcm: &[i16]) -> Result<String> {
        match self {
            SttBackend::Vosk(engine) => engine.transcribe(pcm).await,
            SttBackend::Gemini(engine) => engine.transcribe(pcm).await,
        }
    }
}
