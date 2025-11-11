use serde::{Deserialize, Serialize};
use serde_json::{self, Value, json};
use crate::config;

use crate::config::structs::WakeWordEngine;
use crate::config::structs::SpeechToTextEngine;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub microphone: i32,
    pub voice: String,

    pub wake_word_engine: WakeWordEngine,
    pub speech_to_text_engine: SpeechToTextEngine,

    pub wake_word_threshold: f32,

    pub api_keys: ApiKeys
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            microphone: -1,
            voice: String::from(""),

            wake_word_engine: config::DEFAULT_WAKE_WORD_ENGINE,
            speech_to_text_engine: config::DEFAULT_SPEECH_TO_TEXT_ENGINE,

            wake_word_threshold: 0.5,

            api_keys: ApiKeys {
                picovoice: String::from(""),
                openai: String::from(""),
                gemini: String::from("")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKeys {
    pub picovoice: String,
    pub openai: String,
    pub gemini: String
}

impl Settings {
    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        let json_str = serde_json::to_string(self).ok()?;
        let map: serde_json::Value = serde_json::from_str(&json_str).ok()?;
        
        match key {
            "assistant_voice" | "voice" => {
                serde_json::from_value(map["voice"].clone()).ok()
            },
            "selected_microphone" | "microphone" => {
                serde_json::from_value(map["microphone"].clone()).ok()
            },
            "selected_wake_word_engine" => {
                // Return as string
                match &self.wake_word_engine {
                    WakeWordEngine::Rustpotter => serde_json::from_value(serde_json::json!("rustpotter")).ok(),
                    WakeWordEngine::Vosk => serde_json::from_value(serde_json::json!("vosk")).ok(),
                    WakeWordEngine::Porcupine => serde_json::from_value(serde_json::json!("picovoice")).ok(),
                }
            },
            "api_key__picovoice" => {
                serde_json::from_value(serde_json::json!(self.api_keys.picovoice)).ok()
            },
            "api_key__openai" => {
                serde_json::from_value(serde_json::json!(self.api_keys.openai)).ok()
            },
            "api_key__gemini" => {
                serde_json::from_value(serde_json::json!(self.api_keys.gemini)).ok()
            },
            "wake_word_threshold" => {
                serde_json::from_value(serde_json::json!(self.wake_word_threshold)).ok()
            },
            _ => None
        }
    }

    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: &T) -> Result<(), String> {
        match key {
            "assistant_voice" | "voice" => {
                if let Ok(val) = serde_json::to_string(value) {
                    self.voice = val.trim_matches('"').to_string();
                    Ok(())
                } else {
                    Err("Failed to serialize value".into())
                }
            },
            "selected_microphone" | "microphone" => {
                if let Ok(json) = serde_json::to_value(value) {
                    if let Some(num) = json.as_i64() {
                        self.microphone = num as i32;
                        Ok(())
                    } else if let Some(s) = json.as_str() {
                        self.microphone = s.parse().unwrap_or(-1);
                        Ok(())
                    } else {
                        Err("Invalid microphone value".into())
                    }
                } else {
                    Err("Failed to serialize value".into())
                }
            },
            "selected_wake_word_engine" => {
                if let Ok(json) = serde_json::to_value(value) {
                    if let Some(s) = json.as_str() {
                        self.wake_word_engine = match s.to_lowercase().as_str() {
                            "rustpotter" => WakeWordEngine::Rustpotter,
                            "vosk" => WakeWordEngine::Vosk,
                            "picovoice" => WakeWordEngine::Porcupine,
                            _ => WakeWordEngine::Rustpotter,
                        };
                        Ok(())
                    } else {
                        Err("Invalid wake word engine value".into())
                    }
                } else {
                    Err("Failed to serialize value".into())
                }
            },
            "selected_backend" | "speech_to_text_engine" => {
                if let Ok(json) = serde_json::to_value(value) {
                    if let Some(s) = json.as_str() {
                        self.speech_to_text_engine = match s.to_lowercase().as_str() {
                            "gemini" => SpeechToTextEngine::Gemini,
                            _ => SpeechToTextEngine::Vosk,
                        };
                        Ok(())
                    } else {
                        Err("Invalid speech to text engine value".into())
                    }
                } else {
                    Err("Failed to serialize value".into())
                }
            },
            "api_key__picovoice" => {
                if let Ok(val) = serde_json::to_string(value) {
                    self.api_keys.picovoice = val.trim_matches('"').to_string();
                    Ok(())
                } else {
                    Err("Failed to serialize value".into())
                }
            },
            "api_key__openai" => {
                if let Ok(val) = serde_json::to_string(value) {
                    self.api_keys.openai = val.trim_matches('"').to_string();
                    Ok(())
                } else {
                    Err("Failed to serialize value".into())
                }
            },
            "api_key__gemini" => {
                if let Ok(val) = serde_json::to_string(value) {
                    self.api_keys.gemini = val.trim_matches('"').to_string();
                    Ok(())
                } else {
                    Err("Failed to serialize value".into())
                }
            },
            "wake_word_threshold" => {
                if let Ok(json) = serde_json::to_value(value) {
                    if let Some(num) = json.as_f64() {
                        self.wake_word_threshold = num as f32;
                        Ok(())
                    } else if let Some(s) = json.as_str() {
                        self.wake_word_threshold = s.parse::<f32>().unwrap_or(0.5);
                        Ok(())
                    } else {
                        Err("Invalid wake word threshold".into())
                    }
                } else {
                    Err("Failed to serialize value".into())
                }
            },
            _ => Err("Unknown key".into())
        }
    }
}