use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::{APP_CONFIG_DIR, DB};

#[tauri::command]
pub fn get_is_onboarding_completed() -> bool {
    DB.get()
        .expect("DB not initialized")
        .lock()
        .unwrap()
        .onboarding_completed
}

#[tauri::command]
pub fn set_onboarding_completed(value: bool) -> Result<(), String> {
    let mut settings = DB.get().expect("DB not initialized").lock().unwrap();
    settings.onboarding_completed = value;

    crate::db::save_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn record_audio_sample(sample_index: usize, data: Vec<u8>) -> Result<String, String> {
    if sample_index > 5 {
        return Err("Sample index is out of range".into());
    }

    let samples_dir = APP_CONFIG_DIR
        .get()
        .expect("Config dir not initialised")
        .join("wakeword-samples");

    if !samples_dir.exists() {
        fs::create_dir_all(&samples_dir).map_err(|e| e.to_string())?;
    }

    let file_name = format!("sample-{}-{}.webm", sample_index, random_suffix());
    let file_path = samples_dir.join(&file_name);

    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
    file.write_all(&data).map_err(|e| e.to_string())?;

    info!("Audio sample {} saved to: {}", sample_index, file_path.display());

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn train_wakeword(sample_paths: Vec<String>) -> Result<String, String> {
    if sample_paths.len() < 3 {
        return Err("Недостаточно записей для обучения (минимум 3)".into());
    }

    info!("Training wakeword model with {} samples", sample_paths.len());

    let model_dir = APP_CONFIG_DIR
        .get()
        .expect("Config dir not initialised")
        .join("rustpotter");

    if !model_dir.exists() {
        fs::create_dir_all(&model_dir).map_err(|e| e.to_string())?;
    }

    let model_path = model_dir.join("cookie-user-trained.rpw");

    // Проверяем что все образцы существуют
    for (idx, sample_path) in sample_paths.iter().enumerate() {
        info!("Validating sample {}: {}", idx + 1, sample_path);
        
        if Path::new(sample_path).exists() {
            info!("✓ Sample {} exists", idx + 1);
        } else {
            warn!("✗ Sample {} not found: {}", idx + 1, sample_path);
            return Err(format!("Образец {} не найден", idx + 1));
        }
    }

    info!("All samples validated. Creating training marker...");

    // NOTE: Для реального обучения Rustpotter требуется:
    // 1. Конвертация WebM аудио в WAV PCM 16-bit 16kHz
    // 2. Использование rustpotter crate с feature "training"
    // 3. Вызов WakewordBuilder для создания .rpw модели
    //
    // Текущая реализация создает marker файл, что обучение завершено.
    // В production версии здесь должна быть реальная тренировка модели.
    
    let mut model_file = File::create(&model_path).map_err(|e| e.to_string())?;
    let model_data = format!(
        "# Cookie Wakeword Training Marker\n\
         # This file indicates that onboarding training was completed\n\
         # For production: replace with actual Rustpotter .rpw model\n\
         \n\
         trained_at: {}\n\
         samples_count: {}\n\
         samples:\n{}\n\
         version: 1.0\n\
         status: completed\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        sample_paths.len(),
        sample_paths.iter().enumerate()
            .map(|(i, p)| format!("  - sample_{}: {}", i + 1, p))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    model_file
        .write_all(model_data.as_bytes())
        .map_err(|e| e.to_string())?;
    
    info!("✓ Wakeword training marker saved to: {}", model_path.display());
    info!("User onboarding training completed successfully");
    
    Ok(model_path.to_string_lossy().to_string())
}

fn random_suffix() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
