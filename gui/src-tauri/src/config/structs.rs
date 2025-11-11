use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum WakeWordEngine {
    Rustpotter,
    Vosk,
    Porcupine
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SpeechToTextEngine {
    Vosk,
    Gemini
}

#[derive(PartialEq, Debug)]
pub enum RecorderType {
    Cpal,
}

#[derive(PartialEq, Debug)]
pub enum AudioType {
    Rodio,
    Kira
}

// pub enum TextToSpeechEngine {}

// pub enum IntentRecognitionEngine {}