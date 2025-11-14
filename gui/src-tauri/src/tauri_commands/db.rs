use crate::DB;

#[tauri::command]
pub fn db_read(_key: &str) -> String {
    // Note: This is a stub for compatibility
    // The actual DB implementation uses Settings struct directly
    String::from("")
}

#[tauri::command]
pub fn db_write(_key: &str, _val: &str) -> bool {
    // Note: This is a stub for compatibility
    // The actual DB implementation uses Settings struct directly
    true
}
