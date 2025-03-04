use tauri::{tray::TrayIconBuilder, App, Error};

pub fn run(app: &mut App) -> Result<(), Error> {
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

    Ok(())
}
