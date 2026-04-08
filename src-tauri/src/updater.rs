use tauri::{AppHandle, Emitter};
use tauri_plugin_updater::{Result, UpdaterExt};

#[allow(dead_code)]
pub async fn check(app: AppHandle) -> Result<()> {
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
    }

    Ok(())
}

#[tauri::command]
fn ready(app: AppHandle) {
    app.emit("update-ready", "Update is ready to be installed!")
        .unwrap();
}

#[tauri::command]
pub async fn restart(app: AppHandle) -> Result<()> {
    app.restart();
}
