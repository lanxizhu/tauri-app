#[cfg(target_os = "windows")]
use super::theme;

use super::window_state;
use tauri::{Window, WindowEvent};

pub fn event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::Destroyed => {
            if window.label() == "main" {
                window_state::save(window);
            }
        }
        #[cfg(target_os = "windows")]
        WindowEvent::ThemeChanged(theme) => {
            theme::change(window, theme);
        }
        _ => {}
    }
}
