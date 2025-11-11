use crate::COMMANDS;

#[tauri::command]
pub fn get_commands_list() -> Vec<String> {
    if let Some(commands) = COMMANDS.get() {
        commands.iter()
            .map(|cmd| cmd.path.to_str().unwrap_or("").to_string())
            .collect()
    } else {
        vec![]
    }
}

#[tauri::command]
pub fn execute_command(name: String) -> Result<String, String> {
    // This would execute a command by name
    // For now, return a placeholder
    Ok(format!("Executing command: {}", name))
}

#[tauri::command]
pub fn get_jarvis_phrase(phrase_type: String) -> String {
    match phrase_type.as_str() {
        "greet" => "Я вас слушаю, сэр.".to_string(),
        "ready" => "Готов к работе, сэр.".to_string(),
        "wait" => "Ожидаю команды.".to_string(),
        "executing" => "Выполняю, сэр.".to_string(),
        "done" => "Выполнено, сэр.".to_string(),
        "error" => "Извините, произошла ошибка.".to_string(),
        _ => "".to_string()
    }
}
