use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

use crate::events;

const CMD_RESTART: &str = "restart";
const CMD_SETTINGS: &str = "settings";
const CMD_QUIT: &str = "quit";

pub fn build_tray() -> SystemTray {
    let restart = CustomMenuItem::new(CMD_RESTART, "Перезапуск");
    let settings = CustomMenuItem::new(CMD_SETTINGS, "Настройки");
    let quit = CustomMenuItem::new(CMD_QUIT, "Выход");

    let tray_menu = SystemTrayMenu::new()
        .add_item(restart)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(app: &AppHandle, event: &SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                CMD_RESTART => {
                    events::play("restart", app);
                }
                CMD_SETTINGS => {
                    app.emit_all("open-settings", ()).unwrap();
                }
                CMD_QUIT => {
                    app.exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
