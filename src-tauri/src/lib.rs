mod global_shortcut;
mod single_instance;
mod splash_screen;
mod tray;
mod window_state;

use std::sync::Mutex;
use tauri::{Window, WindowEvent};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tray::init(app).unwrap();
            splash_screen::init(app).unwrap();
            global_shortcut::init(app).unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(window_state::build())
        .plugin(single_instance::init())
        .plugin(global_shortcut::build())
        .manage(Mutex::new(splash_screen::SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        .on_window_event(window_event)
        .invoke_handler(tauri::generate_handler![greet, splash_screen::set_complete])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn window_event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { .. } => {}
        WindowEvent::Destroyed => {
            if window.label() == "main" {
                window_state::save(window);
            }
        }
        _ => {}
    }
}
