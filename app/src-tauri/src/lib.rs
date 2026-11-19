mod commands;
mod commands_tasks;
#[cfg(test)]
mod config_tests;
pub mod db;
pub mod secrets;
pub mod tasks;

use rusqlite::Connection;
use secrets::KeyringStore;
use std::sync::Mutex;
use tauri::Manager;

/// Gemeinsamer Zustand der App: die verschlüsselte Datenbank. Die Sperre nie über Wartezeiten halten.
pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init());
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_process::init());
    }

    // Konnektor für Claude: nur in Entwicklungs-Builds und nur auf diesem Rechner
    // (Standard des Plugins wäre 0.0.0.0, also im ganzen Heimnetz erreichbar)
    #[cfg(feature = "dev-bridge")]
    {
        builder = builder.plugin(tauri_plugin_mcp_bridge::Builder::new().bind_address("127.0.0.1").build());
    }

    builder
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let secrets = KeyringStore::new();
            let conn = db::open_and_migrate(&dir.join(db::DB_FILE), &secrets)?;
            app.manage(AppState { db: Mutex::new(conn) });
            tasks::background::start(app.handle().clone());

            // Updater erst einhängen, wenn ein Update-Schlüssel eingetragen ist (sonst wäre die Konfiguration ungültig).
            #[cfg(desktop)]
            {
                let has_key = app
                    .config()
                    .plugins
                    .0
                    .get("updater")
                    .and_then(|u| u.get("pubkey"))
                    .and_then(|k| k.as_str())
                    .is_some_and(|k| !k.is_empty());
                if has_key {
                    app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands_tasks::task_list_list,
            commands_tasks::task_list_create,
            commands_tasks::task_list_update,
            commands_tasks::task_list_reorder,
            commands_tasks::task_list_delete,
            commands_tasks::task_query,
            commands_tasks::task_counts,
            commands_tasks::task_search,
            commands_tasks::task_create,
            commands_tasks::task_update,
            commands_tasks::task_toggle_done,
            commands_tasks::task_move,
            commands_tasks::task_delete,
            commands_tasks::task_section_list,
            commands_tasks::task_section_create,
            commands_tasks::task_section_rename,
            commands_tasks::task_section_reorder,
            commands_tasks::task_section_delete,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Start von Daedyx Tasks");
}
