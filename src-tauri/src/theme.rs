use raw_window_handle::HasWindowHandle;
use tauri::{App, Manager, Theme};

#[cfg(target_os = "windows")]
use window_vibrancy::apply_mica;

pub fn init(app: &mut App) {
    app.webview_windows().iter().for_each(|window| {
        #[cfg(target_os = "windows")]
        {
            apply_mica(window.1, Some(dark(app)))
                .expect("Unsupported platform! 'apply_mica' is only supported on Windows");
        }
    });
}

pub fn change(window: impl HasWindowHandle, theme: &Theme) {
    let dark = theme == &Theme::Dark;

    #[cfg(target_os = "windows")]
    {
        apply_mica(window, Some(dark))
            .expect("Unsupported platform! 'apply_mica' is only supported on Windows");
    }
}
