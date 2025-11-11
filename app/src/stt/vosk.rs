use std::path::Path;
use anyhow::{Result, bail};
use async_trait::async_trait;

/// Оффлайн STT движок на основе Vosk
/// ПРИМЕЧАНИЕ: Vosk не включен в эту сборку. Для работы Vosk необходимо:
/// 1. Установить библиотеку libvosk в систему
/// 2. Скачать модель (например, vosk-model-small-ru-0.22)
/// 3. Указать путь к модели в конфигурации
pub struct VoskStt {
    _model_path: String,
}

impl VoskStt {
    /// Создает новый STT движок с указанной моделью
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        let path = model_path.as_ref();
        
        log::warn!(
            "⚠️ Vosk STT недоступен в этой сборке. Модель: {}",
            path.display()
        );
        log::info!("💡 Для использования Vosk установите libvosk и пересоберите проект.");
        
        Ok(Self {
            _model_path: path.display().to_string(),
        })
    }

    fn recognize_blocking(_samples: Vec<i16>) -> Result<String> {
        bail!("Vosk STT недоступен в этой сборке. Используйте Gemini API.")
    }
}

#[async_trait]
impl super::SpeechToText for VoskStt {
    async fn transcribe(&mut self, _pcm: &[i16]) -> Result<String> {
        Self::recognize_blocking(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_stub() {
        let result = VoskStt::new("/tmp/model");
        assert!(result.is_ok());
    }
}
