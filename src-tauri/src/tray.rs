use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::TrayIconBuilder,
    App, AppHandle, Error,
};

fn menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            println!("quit menu item was chicked!");
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    }
}

pub fn run(app: &mut App) -> Result<(), Error> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(menu_event)
        .build(app)?;

    Ok(())
}
