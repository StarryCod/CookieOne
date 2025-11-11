use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use vosk::{Model, Recognizer};

/// Оффлайн STT движок на основе Vosk
pub struct VoskStt {
    model: Arc<Model>,
    sample_rate: f32,
}

impl VoskStt {
    /// Создает новый STT движок с указанной моделью
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        let path = model_path.as_ref();
        let path_str = path
            .to_str()
            .context("Путь к модели Vosk должен быть валидной UTF-8 строкой")?;

        let model = Model::new(path_str).context("Не удалось загрузить модель Vosk")?;

        log::info!(
            "Vosk STT инициализирован. Модель: {}",
            path.display()
        );

        Ok(Self {
            model: Arc::new(model),
            sample_rate: 16_000.0,
        })
    }

    fn recognize_blocking(model: Arc<Model>, sample_rate: f32, samples: Vec<i16>) -> Result<String> {
        let mut recognizer = Recognizer::new(&model, sample_rate)
            .context("Не удалось создать распознаватель Vosk")?;

        recognizer.accept_waveform(&samples);

        let result = recognizer.final_result();
        let text = result.multiple().and_then(|res| res.last()).and_then(|single| {
            let alternatives = single.alternatives();
            alternatives.first().map(|alt| alt.text.trim().to_string())
        }).unwrap_or_default();

        log::debug!("Vosk распознал: '{}'", text);

        Ok(text)
    }
}

#[async_trait]
impl super::SpeechToText for VoskStt {
    async fn transcribe(&mut self, pcm: &[i16]) -> Result<String> {
        let model = self.model.clone();
        let sample_rate = self.sample_rate;
        let samples = pcm.to_vec();

        tokio::task::spawn_blocking(move || Self::recognize_blocking(model, sample_rate, samples))
            .await
            .context("Ошибка при выполнении задачи распознавания")?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_invalid_path() {
        let result = VoskStt::new("/path/that/does/not/exist");
        assert!(result.is_err());
    }
}
