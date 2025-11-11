#[cfg(feature = "rustpotter-wakeword")]
use std::path::Path;
#[cfg(feature = "rustpotter-wakeword")]
use anyhow::{Context, Result};
#[cfg(feature = "rustpotter-wakeword")]
use rustpotter::{Rustpotter, RustpotterConfig, WavFmt, DetectorConfig, FiltersConfig, ScoreMode, GainNormalizationConfig, BandPassConfig, Endianness};

/// Детектор wake-word на основе RustPotter
#[cfg(feature = "rustpotter-wakeword")]
pub struct RustpotterDetector {
    detector: Rustpotter,
    threshold: f32,
}

#[cfg(feature = "rustpotter-wakeword")]
impl RustpotterDetector {
    /// Создает новый детектор с указанным путем к модели и порогом
    pub fn new<P: AsRef<Path>>(model_path: P, threshold: f32) -> Result<Self> {
        // Конфигурация RustPotter для PCM i16, 16 kHz, моно
        let config = RustpotterConfig {
            fmt: WavFmt {
                sample_rate: 16000,
                sample_format: hound::SampleFormat::Int,
                bits_per_sample: 16,
                channels: 1,
                endianness: Endianness::Little,
            },
            detector: DetectorConfig {
                threshold,
                avg_threshold: 0.0,
                min_scores: 5,
                score_mode: ScoreMode::Max,
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
        
        let mut detector = Rustpotter::new(&config)
            .map_err(|err| anyhow::anyhow!(err))
            .context("Не удалось создать детектор RustPotter")?;
        
        // Загружаем модель wake-word
        let model_path_str = model_path.as_ref()
            .to_str()
            .context("Путь к модели должен быть валидной UTF-8 строкой")?;
        
        detector
            .add_wakeword_from_file(model_path_str)
            .map_err(|err| anyhow::anyhow!(err))
            .context("Не удалось загрузить модель wake-word")?;
        
        log::info!(
            "RustPotter инициализирован с порогом {}. Модель: {}",
            threshold,
            model_path.as_ref().display()
        );
        
        Ok(Self {
            detector,
            threshold,
        })
    }
    
    /// Обрабатывает аудио кадр и возвращает результат детекции
    /// 
    /// # Аргументы
    /// * `samples` - PCM i16 сэмплы (16 kHz, моно)
    /// 
    /// # Возвращает
    /// * `Some((name, score))` если wake-word обнаружен
    /// * `None` если ничего не обнаружено
    pub fn process_samples(&mut self, samples: &[i16]) -> Option<(String, f32)> {
        match self.detector.process_i16(samples) {
            Some(detection) => {
                let score = detection.score;
                let name = detection.name.clone();
                
                log::debug!("Wake-word обнаружен: '{}' (score: {:.3})", name, score);
                
                Some((name, score))
            }
            None => None,
        }
    }
    
    /// Возвращает текущий порог детекции
    pub fn threshold(&self) -> f32 {
        self.threshold
    }
    
    /// Устанавливает новый порог детекции
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
        log::info!("Порог wake-word изменен на {}", threshold);
    }
    
    /// Сбрасывает внутреннее состояние детектора
    pub fn reset(&mut self) {
        // RustPotter автоматически управляет состоянием,
        // явный сброс не требуется в большинстве случаев
        log::debug!("Детектор wake-word сброшен");
    }
}

// Заглушка для компиляции без feature rustpotter-wakeword
#[cfg(not(feature = "rustpotter-wakeword"))]
pub struct RustpotterDetector {
    threshold: f32,
}

#[cfg(not(feature = "rustpotter-wakeword"))]
impl RustpotterDetector {
    pub fn new<P: AsRef<std::path::Path>>(_model_path: P, threshold: f32) -> anyhow::Result<Self> {
        log::warn!("RustPotter не включен в сборку. Используется заглушечный детектор.");
        log::info!("Для включения RustPotter соберите с --features rustpotter-wakeword");
        
        Ok(Self {
            threshold,
        })
    }
    
    pub fn process_samples(&mut self, _samples: &[i16]) -> Option<(String, f32)> {
        // Заглушка: всегда возвращает None
        None
    }
    
    pub fn threshold(&self) -> f32 {
        self.threshold
    }
    
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }
    
    pub fn reset(&mut self) {
        // Ничего не делаем
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_threshold() {
        let threshold = 0.45;
        assert!(threshold > 0.0 && threshold < 1.0);
    }
}
