use tauri::{
    App, Error, Manager,
    async_runtime::spawn,
    menu::{HELP_SUBMENU_ID, IsMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu},
};
use tauri_plugin_opener::OpenerExt;

use crate::updater;

pub fn init(app: &mut App) -> Result<(), Error> {
    let menu = Menu::default(app.handle())?;

    let separator = PredefinedMenuItem::separator(app.handle())?;
    let help_extension = MenuItem::with_id(
        app.handle(),
        "help_extension",
        "More Help",
        true,
        None::<&str>,
    )?;

    let check_for_updates = MenuItem::with_id(
        app.handle(),
        "check_for_updates",
        "Check for Updates",
        true,
        None::<&str>,
    )?;

    let restart = MenuItem::with_id(app.handle(), "restart", "Restart", true, None::<&str>)?;

    let toggle_devtools = MenuItem::with_id(
        app.handle(),
        "toggle_devtools",
        "Toggle Developer Tools",
        true,
        None::<&str>,
    )?;

    let help_items = [
        &help_extension as &dyn IsMenuItem<_>,
        &separator as &dyn IsMenuItem<_>,
        &check_for_updates as &dyn IsMenuItem<_>,
        &restart as &dyn IsMenuItem<_>,
        &separator as &dyn IsMenuItem<_>,
        &toggle_devtools as &dyn IsMenuItem<_>,
    ];

    if let Some(help_submenu) = menu
        .get(HELP_SUBMENU_ID)
        .and_then(|item| item.as_submenu().cloned())
    {
        help_submenu.append(&separator)?;
        help_submenu.append_items(&help_items)?;
    } else {
        let help_submenu = Submenu::new(app, "Help", true)?;
        help_submenu.append(&separator)?;
        help_submenu.append_items(&help_items)?;
        menu.append(&help_submenu)?;
    }

    let custom_submenu = Submenu::new(app, "Custom", true)?;

    let menu_item1 = MenuItem::new(app.handle(), "Option 1", true, Some("None"))?;
    let menu_item2 = MenuItem::new(app.handle(), "Option 2", true, Some("None"))?;

    custom_submenu.append_items(&[&menu_item1, &menu_item2])?;
    menu.append(&custom_submenu)?;

    app.set_menu(menu)?;
    app.on_menu_event(|app_handle, event| handle_menu_event(app_handle, &event));

    Ok(())
}

fn handle_menu_event(app: &tauri::AppHandle, event: &MenuEvent) {
    match event.id.as_ref() {
        "help_extension" => {
            if let Err(error) = app.opener().open_url("https://tauri.app", None::<&str>) {
                eprintln!("failed to open help page: {error}");
            }
        }
        "check_for_updates" => {
            let handle = app.clone();
            spawn(async move {
                if let Err(error) = updater::check(handle, true).await {
                    eprintln!("failed to check for updates: {error}");
                }
            });
        }
        "restart" => {
            app.restart();
        }
        "toggle_devtools" => {
            if let Some(window) = app.get_webview_window("main") {
                if window.is_devtools_open() {
                    window.close_devtools();
                } else {
                    window.open_devtools();
                }
            }
        }
        _ => {
            println!("Unknown menu item clicked: {:?}", event.id);
        }
    }
}
