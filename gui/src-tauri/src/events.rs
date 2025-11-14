use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventTypes {
    AssistantGreet,
    AssistantWaiting,
    CommandExecuted,
}

impl EventTypes {
    pub fn get(&self) -> &str {
        match self {
            EventTypes::AssistantGreet => "assistant-greet",
            EventTypes::AssistantWaiting => "assistant-waiting",
            EventTypes::CommandExecuted => "command-executed",
        }
    }
}

pub fn init() {
    info!("Events subsystem initialised (stub)");
}

pub fn play(_sound_name: &str, _app_handle: &AppHandle) {
    // Stub: в реальной имплементации здесь воспроизведение звука
    info!("Playing sound: {}", _sound_name);
}
