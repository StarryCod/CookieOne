use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct CommandConfig {
    pub name: String,
}

pub fn fetch_command<'a>(_text: &str, _commands: &'a [CommandConfig]) -> Option<(PathBuf, &'a CommandConfig)> {
    None
}

pub fn execute_command(
    _cmd_path: &PathBuf,
    _cmd_config: &CommandConfig,
    _app_handle: &tauri::AppHandle,
) -> Result<bool, String> {
    Ok(false)
}
