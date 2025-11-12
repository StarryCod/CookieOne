use crate::{config, DB};

#[tauri::command]
pub fn db_read(key: &str) -> String {
    let settings = DB.get().expect("DB not initialized").lock().unwrap();

    match key {
        "assistant_voice" => settings.voice.clone(),
        "selected_wake_word_engine" => match settings.wake_word_engine {
            config::WakeWordEngine::Rustpotter => "rustpotter".to_string(),
            config::WakeWordEngine::Vosk => "vosk".to_string(),
        },
        _ => String::new(),
    }
}

#[tauri::command]
pub fn db_write(key: &str, val: &str) -> bool {
    let mut settings = DB.get().expect("DB not initialized").lock().unwrap();

    match key {
        "assistant_voice" => {
            settings.voice = val.to_string();
        }
        "selected_wake_word_engine" => {
            settings.wake_word_engine = match val.trim().to_lowercase().as_str() {
                "rustpotter" => config::WakeWordEngine::Rustpotter,
                "vosk" => config::WakeWordEngine::Vosk,
                _ => return false,
            };
        }
        _ => return false,
    }

    crate::db::save_settings(&settings).is_ok()
}
