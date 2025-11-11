mod vosk;
mod gemini;

use once_cell::sync::OnceCell;
use crate::config;

use crate::config::structs::SpeechToTextEngine;

static STT_TYPE: OnceCell<SpeechToTextEngine> = OnceCell::new();

pub fn init() -> Result<(), ()> {
    if !STT_TYPE.get().is_none() {return Ok(());} // already initialized

    // set default stt type
    // @TODO. Make it configurable?
    STT_TYPE.set(config::DEFAULT_SPEECH_TO_TEXT_ENGINE).unwrap();

    // load given recorder
    match STT_TYPE.get().unwrap() {
        SpeechToTextEngine::Vosk => {
            // Init Vosk
            info!("Initializing Vosk STT backend.");
            vosk::init_vosk();
            info!("STT backend initialized.");
        }
        SpeechToTextEngine::Gemini => {
            info!("Initializing Gemini STT backend.");
            if gemini::init().is_err() {
                warn!("Gemini STT initialization failed. Falling back to Vosk.");
                STT_TYPE.set(SpeechToTextEngine::Vosk).ok();
                vosk::init_vosk();
            }
        }
    }

    Ok(())
}

pub fn recognize(data: &[i16], partial: bool) -> Option<String> {
    match STT_TYPE.get().unwrap() {
        SpeechToTextEngine::Vosk => {
            vosk::recognize(data, partial)
        }
        SpeechToTextEngine::Gemini => {
            gemini::recognize(data, partial)
        }
    }
}