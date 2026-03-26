use super::theme;
use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuBuilder, MenuEvent, MenuItem, SubmenuBuilder},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Error, Manager, Theme, Wry,
};

const TRAY_ID: &str = "main-tray";

fn menu_init(app: &App) -> Result<Menu<Wry>, Error> {
    let dark = CheckMenuItem::with_id(app, "dark", "Dark", true, theme::dark(app), None::<&str>)?;

    let light =
        CheckMenuItem::with_id(app, "light", "Light", true, theme::light(app), None::<&str>)?;

    let theme_menus = SubmenuBuilder::new(app, "Theme")
        .item(&dark)
        .item(&light)
        .build()?;

    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = MenuBuilder::new(app)
        .item(&theme_menus)
        .separator()
        .item(&quit_i)
        .build()?;

    let handle = move |app_handle: &AppHandle, event: MenuEvent| match event.id.as_ref() {
        "dark" => {
            app_handle.set_theme(Some(Theme::Dark));
            light.set_checked(false).expect("Change check error");
            tray_icon_change(app_handle, Theme::Dark);
        }
        "light" => {
            app_handle.set_theme(Some(Theme::Light));
            dark.set_checked(false).expect("Change check error");
            tray_icon_change(app_handle, Theme::Light);
        }
        "quit" => {
            app_handle.exit(0);
        }
        _ => {}
    };

    app.on_menu_event(handle);

    Ok(menu)
}

fn tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("left click pressed and released");
            let app = tray.app_handle();
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(true) = window.is_minimizable() {
                    window.unminimize().unwrap();
                }
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }

        _ => {
            // println!("unhandled event {event:?}");
        }
    }
}

fn tray_icon_change(app_handle: &AppHandle, theme: Theme) {
    #[cfg(target_os = "windows")]
    {
        let icon = if theme == Theme::Dark {
            Image::from_bytes(include_bytes!("../icons/tray_black.png")).ok()
        } else {
            Image::from_bytes(include_bytes!("../icons/tray.png")).ok()
        };

        app_handle
            .tray_by_id(TRAY_ID)
            .expect("Failed to get tray")
            .set_icon(icon)
            .expect("Failed to change tray icon");
    }
}

pub fn init(app: &mut App) -> Result<(), Error> {
    let menu = menu_init(app)?;

    let tray = TrayIconBuilder::<Wry>::with_id(TRAY_ID)
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(tray_icon_event)
        .build(app)?;

    #[cfg(target_os = "windows")]
    {
        tray.set_icon(app.default_window_icon().unwrap().clone())
    }

    #[cfg(target_os = "macos")]
    {
        let icon = Image::from_bytes(include_bytes!("../icons/tray.png")).ok();
        let _ = tray.set_icon(icon);
    }

    Ok(())
}
