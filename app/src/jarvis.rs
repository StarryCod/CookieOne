use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

/// Типы фраз JARVIS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JarvisPhrases {
    /// Подтверждение получения команды ("Да, сэр.")
    pub ack: Vec<String>,
    
    /// Обработка запроса ("Работаю над этим, сэр.")
    pub processing: Vec<String>,
    
    /// Завершение задачи ("Задача выполнена, сэр.")
    pub done: Vec<String>,
    
    /// Ошибка ("Произошла ошибка, сэр.")
    pub error: Vec<String>,
    
    /// Активация по wake-word ("Я вас слушаю, сэр.")
    pub wake: Vec<String>,
}

impl Default for JarvisPhrases {
    fn default() -> Self {
        Self {
            ack: vec![
                "Да, сэр.".to_string(),
                "Слушаю, сэр.".to_string(),
                "Готов к выполнению, сэр.".to_string(),
            ],
            processing: vec![
                "Работаю над этим, сэр.".to_string(),
                "Выполняю, сэр.".to_string(),
                "Обрабатываю ваш запрос, сэр.".to_string(),
            ],
            done: vec![
                "Задача выполнена, сэр.".to_string(),
                "Готово, сэр.".to_string(),
                "Все сделано, сэр.".to_string(),
            ],
            error: vec![
                "Произошла ошибка, сэр.".to_string(),
                "Мне нужно уточнение, сэр.".to_string(),
                "Это действие недоступно, сэр.".to_string(),
            ],
            wake: vec![
                "Я вас слушаю, сэр.".to_string(),
                "Да, сэр?".to_string(),
                "На связи, сэр.".to_string(),
            ],
        }
    }
}

impl JarvisPhrases {
    /// Загружает фразы из JSON файла
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .context("Не удалось прочитать файл фраз JARVIS")?;
        
        let phrases: JarvisPhrases = serde_json::from_str(&content)
            .context("Не удалось распарсить фразы JARVIS")?;
        
        Ok(phrases)
    }
    
    /// Сохраняет фразы в JSON файл
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Не удалось сериализовать фразы JARVIS")?;
        
        fs::write(path.as_ref(), content)
            .context("Не удалось записать файл фраз JARVIS")?;
        
        Ok(())
    }
    
    /// Загружает фразы или создает дефолтные
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            Self::load(path)
        } else {
            let phrases = Self::default();
            
            // Создаем родительские директории если нужно
            if let Some(parent) = path.as_ref().parent() {
                fs::create_dir_all(parent)?;
            }
            
            phrases.save(&path)?;
            Ok(phrases)
        }
    }
    
    /// Возвращает случайную фразу подтверждения
    pub fn get_random_ack(&self) -> &str {
        self.get_random(&self.ack)
    }
    
    /// Возвращает случайную фразу обработки
    pub fn get_random_processing(&self) -> &str {
        self.get_random(&self.processing)
    }
    
    /// Возвращает случайную фразу завершения
    pub fn get_random_done(&self) -> &str {
        self.get_random(&self.done)
    }
    
    /// Возвращает случайную фразу ошибки
    pub fn get_random_error(&self) -> &str {
        self.get_random(&self.error)
    }
    
    /// Возвращает случайную фразу активации
    pub fn get_random_wake(&self) -> &str {
        self.get_random(&self.wake)
    }
    
    /// Возвращает случайный элемент из массива
    fn get_random<'a>(&self, phrases: &'a [String]) -> &'a str {
        if phrases.is_empty() {
            return "";
        }
        
        let index = fastrand::usize(..phrases.len());
        phrases[index].as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_phrases() {
        let phrases = JarvisPhrases::default();
        assert!(!phrases.ack.is_empty());
        assert!(!phrases.wake.is_empty());
    }
    
    #[test]
    fn test_random_selection() {
        let phrases = JarvisPhrases::default();
        let ack = phrases.get_random_ack();
        assert!(!ack.is_empty());
    }
}
