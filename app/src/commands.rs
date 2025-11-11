use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

/// Описание действия команды
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CommandAction {
    /// Запуск внешнего процесса (CLI, скрипт и т.п.)
    RunProcess {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        working_dir: Option<String>,
    },
    /// Проигрывание заранее записанного аудио-файла
    PlayAudio {
        file: String,
    },
    /// Возврат текстового ответа (для TTS)
    RespondText {
        text: String,
    },
}

/// Команда ассистента
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDefinition {
    /// Уникальный идентификатор команды
    pub id: String,
    /// Короткое название
    pub name: String,
    /// Краткое описание
    pub description: Option<String>,
    /// Путь к иконке (для UI)
    pub icon: Option<String>,
    /// Список ключевых фраз, которые активируют команду
    pub keywords: Vec<String>,
    /// Действие команды
    pub action: CommandAction,
}

/// Список команд
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandLibrary {
    pub commands: Vec<CommandDefinition>,
}

impl CommandLibrary {
    /// Загружает список команд из JSON файла
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .context("Не удалось прочитать файл команд")?;
        
        let library: CommandLibrary = serde_json::from_str(&content)
            .context("Не удалось распарсить команды")?;
        
        Ok(library)
    }
    
    /// Сохраняет команды в JSON файл
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Не удалось сериализовать команды")?;
        
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)
                .context("Не удалось создать директорию для команд")?;
        }
        
        fs::write(path.as_ref(), content)
            .context("Не удалось записать файл команд")?;
        
        Ok(())
    }
    
    /// Загружает команды или создает дефолтную библиотеку
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            Self::load(path)
        } else {
            let library = CommandLibrary::default_library();
            library.save(&path)?;
            Ok(library)
        }
    }
    
    /// Находит команду по ключевому слову в распознанном тексте
    pub fn find_by_text<'a>(&'a self, text: &str) -> Option<&'a CommandDefinition> {
        let normalized = text.to_lowercase();
        
        self.commands.iter().find(|command| {
            command.keywords.iter().any(|keyword| {
                let keyword = keyword.to_lowercase();
                normalized.contains(&keyword)
            })
        })
    }
    
    /// Дефолтная библиотека команд (можно расширить позже)
    fn default_library() -> Self {
        let commands = vec![
            CommandDefinition {
                id: "greeting".to_string(),
                name: "Приветствие".to_string(),
                description: Some("Отвечает на приветствие".to_string()),
                icon: None,
                keywords: vec!["привет".to_string(), "здравствуй".to_string()],
                action: CommandAction::RespondText {
                    text: "Здравствуйте, сэр. Чем могу помочь?".to_string(),
                },
            },
            CommandDefinition {
                id: "time".to_string(),
                name: "Текущее время".to_string(),
                description: Some("Сообщает текущее время".to_string()),
                icon: None,
                keywords: vec!["который час".to_string(), "время".to_string()],
                action: CommandAction::RunProcess {
                    command: "cmd".to_string(),
                    args: vec!["/C".to_string(), "time /T".to_string()],
                    working_dir: None,
                },
            },
        ];
        
        CommandLibrary { commands }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_by_text() {
        let library = CommandLibrary::default_library();
        let command = library.find_by_text("привет, ассистент");
        assert!(command.is_some());
        assert_eq!(command.unwrap().id, "greeting");
    }
}
