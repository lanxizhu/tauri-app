mod global_shortcut;
mod splash_screen;
mod tray;
mod window_state;

use std::sync::Mutex;
use tauri::{Manager, Window, WindowEvent};

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
            global_shortcut::init(app).unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(window_state::get_flags())
                .skip_initial_state("splashscreen")
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let window = app.get_webview_window("main").expect("no main window");
            if let Ok(true) = window.is_minimized() {
                window.unminimize().unwrap();
            }
            if let Ok(false) = window.is_visible() {
                window.show().unwrap();
            }
            window.set_focus().unwrap();
        }))
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
