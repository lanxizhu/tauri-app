use tauri::{
    AppHandle, Emitter,
    menu::{HELP_SUBMENU_ID, MenuItemKind},
};
use tauri_plugin_updater::{Result, UpdaterExt};

#[allow(dead_code)]
pub async fn check(app: AppHandle, need_emit: bool) -> Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update is ready to be installed");
        // restart(app).await?;
        ready(app);
    } else {
        println!("no update available");
        if need_emit {
            app.emit("update-not-available", "No update available!")
                .unwrap();
        }
    }

    Ok(())
}

#[tauri::command]
fn ready(app: AppHandle) {
    app.emit("update-ready", "Update is ready to be installed!")
        .unwrap();

    if let Some(menu) = app.menu() {
        if let Some(item) = menu
            .get(HELP_SUBMENU_ID)
            .and_then(|item| item.as_submenu().cloned())
            .and_then(|submenu| submenu.get("restart"))
        {
            if let MenuItemKind::MenuItem(menu_item) = item {
                let _ = menu_item.set_text("Restart To Update");
            }
        }
    }
}

#[tauri::command]
pub async fn restart(app: AppHandle) -> Result<()> {
    app.restart();
}
