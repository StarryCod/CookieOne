use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

use super::SpeechToText;
use crate::config::GEMINI_ENDPOINT;

pub struct GeminiAudioEngine {
    client: Client,
    api_key: String,
}

impl GeminiAudioEngine {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }
}

impl SpeechToText for GeminiAudioEngine {
    fn transcribe(&self, pcm: &[i16]) -> Result<Option<String>> {
        let bytes: Vec<u8> = pcm.iter().flat_map(|sample| sample.to_le_bytes()).collect();
        let payload = STANDARD.encode(bytes);

        let request_body = json!({
            "contents": [{
                "parts": [
                    {
                        "inline_data": {
                            "mime_type": "audio/pcm;rate=16000",
                            "data": payload
                        }
                    },
                    {
                        "text": "Transcribe this audio."
                    }
                ]
            }]
        });

        let url = format!("{}?key={}", GEMINI_ENDPOINT, self.api_key);
        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Gemini API error: {}", response.text()?));
        }

        let body: GeminiResponse = response.json()?;
        let text = body
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.trim().to_string());

        Ok(text)
    }
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<TextPart>,
}

#[derive(Debug, Deserialize)]
struct TextPart {
    text: String,
}
