use tauri::{AppHandle, Manager, Window};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

pub fn get_flags() -> StateFlags {
    StateFlags::all() & !StateFlags::VISIBLE
}

pub fn restore(app: AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.restore_state(get_flags()).unwrap();
    println!("restored {:?} window state...", window.label());
}

pub fn save(window: &Window) {
    println!("{:?} window destroyed, saving state...", window.label());
    if let Err(err) = window.app_handle().save_window_state(get_flags()) {
        eprintln!("Error saving window state: {:?}", err);
    }
}
