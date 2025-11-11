use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

/// Online STT через Gemini API
#[derive(Debug)]
pub struct GeminiStt {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Part {
    Text { text: String },
    InlineData { inline_data: InlineData },
}

#[derive(Serialize)]
struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

impl GeminiStt {
    /// Создает новый STT движок с указанным API ключом
    pub fn new(api_key: Option<String>) -> Result<Self> {
        let api_key = api_key.context("API ключ Gemini не установлен")?;
        
        if api_key.is_empty() {
            bail!("API ключ Gemini пустой");
        }
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Не удалось создать HTTP клиент")?;
        
        log::info!("Gemini STT инициализирован");
        
        Ok(Self { api_key, client })
    }
    
    /// Распознает речь через Gemini API
    pub async fn recognize(&self, samples: &[i16]) -> Result<String> {
        // Конвертируем PCM i16 в WAV формат
        let wav_data = self.pcm_to_wav(samples)?;
        
        // Кодируем в base64
        let base64_audio = general_purpose::STANDARD.encode(&wav_data);
        
        // Формируем запрос
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![
                    Part::Text {
                        text: "Распознай речь из этого аудио и верни только текст без дополнительных пояснений.".to_string(),
                    },
                    Part::InlineData {
                        inline_data: InlineData {
                            mime_type: "audio/wav".to_string(),
                            data: base64_audio,
                        },
                    },
                ],
            }],
        };
        
        // Отправляем запрос к Gemini API
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
            self.api_key
        );
        
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Не удалось отправить запрос к Gemini API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            bail!("Gemini API вернул ошибку: {}", error_text);
        }
        
        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Не удалось распарсить ответ Gemini API")?;
        
        // Извлекаем текст из ответа
        let text = gemini_response
            .candidates
            .get(0)
            .and_then(|c| c.content.parts.get(0))
            .map(|p| p.text.trim().to_string())
            .unwrap_or_default();
        
        log::debug!("Gemini распознал: '{}'", text);
        
        Ok(text)
    }
    
    /// Конвертирует PCM i16 сэмплы в WAV формат
    fn pcm_to_wav(&self, samples: &[i16]) -> Result<Vec<u8>> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        
        let mut writer = hound::WavWriter::new(&mut cursor, spec)
            .context("Не удалось создать WAV writer")?;
        
        for &sample in samples {
            writer.write_sample(sample)
                .context("Не удалось записать сэмпл в WAV")?;
        }
        
        writer.finalize()
            .context("Не удалось финализировать WAV")?;
        
        Ok(cursor.into_inner())
    }
}

#[async_trait]
impl super::SpeechToText for GeminiStt {
    async fn transcribe(&mut self, pcm: &[i16]) -> Result<String> {
        self.recognize(pcm).await
    }
}
