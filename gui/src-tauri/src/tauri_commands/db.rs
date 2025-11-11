use crate::DB;

#[tauri::command]
pub fn db_read(key: &str) -> String {
    if let Some(settings) = DB.get() {
        let guard = settings.lock().unwrap();
        match guard.get::<String>(key) {
            Some(value) => value,
            None => String::from("")
        }
    } else {
        String::from("")
    }
}

#[tauri::command]
pub fn db_write(key: &str, val: &str) -> bool {
    if let Some(settings) = DB.get() {
        let mut guard = settings.lock().unwrap();
        if guard.set(key, &val).is_ok() {
            if crate::db::save_settings(&*guard).is_ok() {
                return true;
            }
        }
    }
    false
}
