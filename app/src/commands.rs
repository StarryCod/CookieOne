use crate::{audio, config, APP_DIR};
use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

static COMMANDS: OnceCell<Vec<Command>> = OnceCell::new();

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    pub phrases: Vec<String>,
    pub action: CommandAction,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum CommandAction {
    #[serde(rename = "shell")]
    Shell { command: String },
    #[serde(rename = "sound")]
    Sound { file: String },
}

pub fn init() -> Result<()> {
    let cfg = config::config();
    let path = APP_DIR.join(&cfg.commands_path);
    let contents = fs::read_to_string(&path)?;
    let commands: Vec<Command> = serde_json::from_str(&contents)?;

    COMMANDS
        .set(commands)
        .map_err(|_| anyhow::anyhow!("Commands already initialized"))?;

    info!(
        "Loaded {} commands",
        COMMANDS.get().map(|c| c.len()).unwrap_or(0)
    );
    Ok(())
}

pub fn match_command(text: &str) -> Option<Command> {
    let commands = COMMANDS.get()?;
    let lower = text.to_lowercase();

    commands
        .iter()
        .find(|cmd| {
            cmd.phrases
                .iter()
                .any(|phrase| lower.contains(&phrase.to_lowercase()))
        })
        .cloned()
}

pub fn execute(command: &Command) -> Result<()> {
    match &command.action {
        CommandAction::Shell { command } => {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()?;
            Ok(())
        }
        CommandAction::Sound { file } => {
            let path = APP_DIR.join(file);
            audio::play(format!("Playing sound {}", path.display()))
        }
    }
}
