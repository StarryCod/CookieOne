use anyhow::{bail, Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig};
use crossbeam::channel::{self, Receiver, Sender};
use parking_lot::Mutex;
use std::sync::Arc;

/// Аудио-пайплайн для захвата микрофонного ввода
pub struct AudioPipeline {
    #[allow(dead_code)]
    host: Host,
    device: Device,
    config: StreamConfig,
    stream: Option<Stream>,
    sender: Option<Sender<Vec<i16>>>,
    is_running: Arc<Mutex<bool>>,
}

impl AudioPipeline {
    /// Создает новый аудио-пайплайн
    pub fn new(device_index: usize) -> Result<Self> {
        let host = cpal::default_host();
        
        // Получаем список устройств ввода
        let devices: Vec<Device> = host
            .input_devices()
            .context("Не удалось получить список устройств ввода")?
            .collect();
        
        if devices.is_empty() {
            bail!("Не найдено ни одного устройства ввода");
        }
        
        // Выбираем устройство по индексу или дефолтное
        let device = if device_index < devices.len() {
            devices.into_iter().nth(device_index).unwrap()
        } else {
            host.default_input_device()
                .context("Не удалось получить дефолтное устройство ввода")?
        };
        
        let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());
        log::info!("Выбрано аудио устройство: {}", device_name);
        
        // Получаем конфигурацию устройства
        let mut supported_configs = device
            .supported_input_configs()
            .context("Не удалось получить поддерживаемые конфигурации")?;
        
        // Ищем конфигурацию с частотой 16 kHz
        let config = supported_configs
            .find(|c| {
                c.min_sample_rate().0 <= 16000 && c.max_sample_rate().0 >= 16000
            })
            .context("Устройство не поддерживает частоту дискретизации 16 kHz")?
            .with_sample_rate(cpal::SampleRate(16000))
            .into();
        
        log::info!("Конфигурация аудио: {:?}", config);
        
        Ok(Self {
            host,
            device,
            config,
            stream: None,
            sender: None,
            is_running: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Список доступных устройств ввода
    #[allow(dead_code)]
    pub fn list_devices() -> Result<Vec<String>> {
        let host = cpal::default_host();
        let devices: Vec<String> = host
            .input_devices()
            .context("Не удалось получить список устройств")?
            .filter_map(|d| d.name().ok())
            .collect();
        
        Ok(devices)
    }
    
    /// Запускает захват аудио
    pub fn start(&mut self) -> Result<Receiver<Vec<i16>>> {
        if *self.is_running.lock() {
            bail!("Аудио-пайплайн уже запущен");
        }
        
        let (tx, rx) = channel::unbounded();
        let tx_clone = tx.clone();
        
        let is_running = self.is_running.clone();
        *is_running.lock() = true;
        
        let channels = self.config.channels as usize;
        
        // Создаем stream для захвата аудио
        let stream = self.device.build_input_stream(
            &self.config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Конвертируем f32 в i16 и делаем моно
                let mut mono_samples = Vec::with_capacity(data.len() / channels);
                
                for chunk in data.chunks(channels) {
                    // Усредняем каналы для получения моно
                    let avg: f32 = chunk.iter().sum::<f32>() / channels as f32;
                    
                    // Конвертируем в i16
                    let sample = (avg * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
                    mono_samples.push(sample);
                }
                
                // Отправляем в канал
                if !mono_samples.is_empty() {
                    let _ = tx_clone.send(mono_samples);
                }
            },
            move |err| {
                log::error!("Ошибка аудио-потока: {}", err);
                *is_running.lock() = false;
            },
            None,
        )
        .context("Не удалось создать аудио-поток")?;
        
        stream.play().context("Не удалось запустить аудио-поток")?;
        
        self.stream = Some(stream);
        self.sender = Some(tx);
        
        log::info!("Аудио-пайплайн запущен");
        
        Ok(rx)
    }
    
    /// Останавливает захват аудио
    pub fn stop(&mut self) {
        *self.is_running.lock() = false;
        
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
        
        self.sender = None;
        
        log::info!("Аудио-пайплайн остановлен");
    }
    
    /// Проверяет, запущен ли пайплайн
    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        *self.is_running.lock()
    }
}

impl Drop for AudioPipeline {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Сохраняет PCM i16 сэмплы в WAV файл
#[allow(dead_code)]
pub fn save_to_wav<P: AsRef<std::path::Path>>(path: P, samples: &[i16]) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = hound::WavWriter::create(path, spec)
        .context("Не удалось создать WAV файл")?;
    
    for &sample in samples {
        writer.write_sample(sample)
            .context("Не удалось записать сэмпл в WAV")?;
    }
    
    writer.finalize()
        .context("Не удалось финализировать WAV файл")?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_list_devices() {
        let devices = AudioPipeline::list_devices();
        assert!(devices.is_ok());
    }
}
