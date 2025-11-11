use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

/// Конфигурация приложения Cookie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Порог активации wake-word (0.0 - 1.0)
    pub wake_word_threshold: f32,
    
    /// Путь к модели wake-word RustPotter
    pub wake_word_path: String,
    
    /// Настройки STT движка
    pub stt_backend: SttBackendConfig,
    
    /// API ключ для Gemini (опционально)
    pub gemini_api_key: Option<String>,
    
    /// Путь к файлу с фразами JARVIS
    pub jarvis_phrases: String,
    
    /// Путь к файлу команд
    pub commands_path: String,
    
    /// Индекс устройства для захвата аудио
    pub listening_device: usize,
}

/// Конфигурация STT движка
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SttBackendConfig {
    Vosk {
        model_path: String,
    },
    Gemini,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wake_word_threshold: 0.45,
            wake_word_path: "assets/wakeword/cookie.rpw".to_string(),
            stt_backend: SttBackendConfig::Vosk {
                model_path: "vosk/model_small".to_string(),
            },
            gemini_api_key: None,
            jarvis_phrases: "assets/phrases/jarvis_style.json".to_string(),
            commands_path: "commands/commands.json".to_string(),
            listening_device: 0,
        }
    }
}

impl Config {
    /// Загружает конфигурацию из файла
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .context("Не удалось прочитать файл конфигурации")?;
        
        let config: Config = serde_json::from_str(&content)
            .context("Не удалось распарсить конфигурацию")?;
        
        Ok(config)
    }
    
    /// Сохраняет конфигурацию в файл
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Не удалось сериализовать конфигурацию")?;
        
        fs::write(path.as_ref(), content)
            .context("Не удалось записать файл конфигурации")?;
        
        Ok(())
    }
    
    /// Загружает конфигурацию или создает с дефолтными значениями
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            Self::load(path)
        } else {
            let config = Self::default();
            
            // Создаем родительские директории если нужно
            if let Some(parent) = path.as_ref().parent() {
                fs::create_dir_all(parent)?;
            }
            
            config.save(&path)?;
            Ok(config)
        }
    }
}

/// Получает путь к директории конфигурации приложения
pub fn get_config_dir() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("com", "cookie", "assistant")
        .context("Не удалось определить директорию конфигурации")?;
    
    let config_dir = dirs.config_dir();
    
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }
    
    Ok(config_dir.to_path_buf())
}

/// Получает путь к файлу конфигурации
pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.wake_word_threshold, 0.45);
        assert!(config.gemini_api_key.is_none());
    }
}
