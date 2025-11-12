use std::fs::{self, File};
use std::io::Write;

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
        .join("models");

    if !model_dir.exists() {
        fs::create_dir_all(&model_dir).map_err(|e| e.to_string())?;
    }

    let model_path = model_dir.join("cookie-trained.rpw");

    // TODO: Реальная имплементация обучения Rustpotter модели
    // Сейчас создаём placeholder файл
    let mut model_file = File::create(&model_path).map_err(|e| e.to_string())?;
    model_file
        .write_all(b"cookie wakeword model placeholder - trained from user samples")
        .map_err(|e| e.to_string())?;

    info!("Wakeword model saved to: {}", model_path.display());

    Ok(model_path.to_string_lossy().to_string())
}

fn random_suffix() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
