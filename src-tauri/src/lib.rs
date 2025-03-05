mod splash_screen;
mod tray;

use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let _tray = tray::run(app);
            let _splash_screen = splash_screen::run(app);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(splash_screen::SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        .invoke_handler(tauri::generate_handler![greet, splash_screen::set_complete])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
