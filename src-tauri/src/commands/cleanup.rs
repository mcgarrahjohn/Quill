//! Commands for the optional on-device cleanup ("Remove filler words").

use crate::cleanup::{CleanupManager, CleanupStatus};
use crate::settings::{get_settings, write_settings};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
#[specta::specta]
pub fn get_cleanup_status(manager: State<'_, Arc<CleanupManager>>) -> CleanupStatus {
    manager.status()
}

/// Turns the setting on or off. Turning it on downloads the model (once) if it
/// is missing; turning it off frees the memory right away.
#[tauri::command]
#[specta::specta]
pub fn change_smart_cleanup_setting(
    app: AppHandle,
    manager: State<'_, Arc<CleanupManager>>,
    enabled: bool,
) -> Result<(), String> {
    let mut settings = get_settings(&app);
    settings.smart_cleanup_enabled = enabled;
    write_settings(&app, settings);

    if enabled {
        if manager.files_ready() {
            manager.preload();
        } else {
            manager.start_download();
        }
    } else {
        manager.cancel_download();
        manager.unload();
    }

    let _ = app.emit(
        "settings-changed",
        serde_json::json!({ "setting": "smart_cleanup_enabled", "value": enabled }),
    );
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn start_cleanup_download(manager: State<'_, Arc<CleanupManager>>) {
    manager.start_download();
}

#[tauri::command]
#[specta::specta]
pub fn cancel_cleanup_download(manager: State<'_, Arc<CleanupManager>>) {
    manager.cancel_download();
}

/// Frees the disk space used by the cleanup model (and switches nothing else).
#[tauri::command]
#[specta::specta]
pub fn delete_cleanup_model(manager: State<'_, Arc<CleanupManager>>) -> Result<(), String> {
    manager.delete_files()
}
