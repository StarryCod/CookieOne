use crate::{config, APP_DIR};
use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

static PHRASES: OnceCell<JarvisPhrases> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct JarvisPhrases {
    ack: Vec<String>,
    processing: Vec<String>,
    done: Vec<String>,
    error: Vec<String>,
    wake: Vec<String>,
}

impl JarvisPhrases {
    fn get_random<'a>(&self, phrases: &'a [String]) -> &'a str {
        if phrases.is_empty() {
            "Yes, sir."
        } else {
            let idx = fastrand::usize(..phrases.len());
            &phrases[idx]
        }
    }

    pub fn wake(&self) -> &str {
        self.get_random(&self.wake)
    }

    pub fn ack(&self) -> &str {
        self.get_random(&self.ack)
    }

    pub fn processing(&self) -> &str {
        self.get_random(&self.processing)
    }

    pub fn done(&self) -> &str {
        self.get_random(&self.done)
    }

    pub fn error(&self) -> &str {
        self.get_random(&self.error)
    }
}

pub fn init() -> Result<()> {
    let cfg = config::config();
    let path = APP_DIR.join(&cfg.jarvis_phrases);

    let contents = fs::read_to_string(&path)?;
    let phrases: JarvisPhrases = serde_json::from_str(&contents)?;

    PHRASES
        .set(phrases)
        .map_err(|_| anyhow::anyhow!("Phrases already initialized"))?;

    info!("Persona initialized");
    Ok(())
}

pub fn get_wake_phrase() -> &'static str {
    PHRASES.get().map(|p| p.wake()).unwrap_or("Yes, sir?")
}

pub fn get_ack_phrase() -> &'static str {
    PHRASES.get().map(|p| p.ack()).unwrap_or("Yes, sir.")
}

pub fn get_processing_phrase() -> &'static str {
    PHRASES
        .get()
        .map(|p| p.processing())
        .unwrap_or("On it, sir.")
}

pub fn get_done_phrase() -> &'static str {
    PHRASES.get().map(|p| p.done()).unwrap_or("Completed, sir.")
}

pub fn get_error_phrase() -> &'static str {
    PHRASES
        .get()
        .map(|p| p.error())
        .unwrap_or("Sir, an error has occurred.")
}
