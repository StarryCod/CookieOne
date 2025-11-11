use crate::DB;
use serde_json::json;

#[tauri::command]
pub fn get_config() -> String {
    if let Some(settings) = DB.get() {
        let guard = settings.lock().unwrap();
        serde_json::to_string(&*guard).unwrap_or_default()
    } else {
        "{}".to_string()
    }
}

#[tauri::command]
pub fn save_gemini_api_key(key: String) -> Result<bool, String> {
    if let Some(settings) = DB.get() {
        let mut guard = settings.lock().unwrap();
        guard.api_keys.gemini = key;
        
        // Save to file
        match crate::db::save_settings(&*guard) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Failed to save settings: {}", e))
        }
    } else {
        Err("Settings not initialized".into())
    }
}

#[tauri::command]
pub fn set_listening_device(device_index: i32) -> Result<bool, String> {
    if let Some(settings) = DB.get() {
        let mut guard = settings.lock().unwrap();
        guard.microphone = device_index;
        
        // Save to file
        match crate::db::save_settings(&*guard) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Failed to save settings: {}", e))
        }
    } else {
        Err("Settings not initialized".into())
    }
}
