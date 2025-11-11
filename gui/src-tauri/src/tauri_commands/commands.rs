use crate::{COMMANDS, assistant_commands};
use tauri::Manager;

#[tauri::command]
pub fn get_commands_list() -> Vec<String> {
    if let Some(commands) = COMMANDS.get() {
        let mut result = vec![];
        for cmd in commands.iter() {
            for scmd in &cmd.commands.list {
                for phrase in &scmd.phrases {
                    result.push(phrase.clone());
                }
            }
        }
        result
    } else {
        vec![]
    }
}

#[tauri::command]
pub fn execute_command_by_text(text: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    if let Some(commands) = COMMANDS.get() {
        // Try to find matching command
        if let Some((cmd_path, cmd_config)) = assistant_commands::fetch_command(&text, commands) {
            info!("Found command for text: {}", text);
            
            // Emit command start event
            app_handle.emit_all("command-start", ()).ok();
            
            // Execute the command
            match assistant_commands::execute_command(cmd_path, cmd_config) {
                Ok(chain) => {
                    // Emit command end event
                    app_handle.emit_all("command-end", ()).ok();
                    
                    Ok(format!("Command executed successfully. Chain: {}", chain))
                },
                Err(e) => {
                    // Emit error event
                    app_handle.emit_all("error-occurred", e.clone()).ok();
                    Err(e)
                }
            }
        } else {
            Err("Command not found".to_string())
        }
    } else {
        Err("Commands not initialized".to_string())
    }
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
