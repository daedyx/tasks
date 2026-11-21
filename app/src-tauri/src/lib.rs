mod background;
mod commands;
mod commands_tasks;
#[cfg(test)]
mod config_tests;
pub mod db;
pub mod secrets;
mod startup;
mod startup_dialog;
#[cfg(test)]
mod startup_tests;
mod startup_text;
pub mod tasks;

use rusqlite::Connection;
use secrets::KeyringStore;
use std::sync::Mutex;
use tauri::Manager;

/// Gemeinsamer Zustand der App: die verschlüsselte Datenbank. Die Sperre nie über Wartezeiten halten.
pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg(desktop)]
fn show_main(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init());
    #[cfg(desktop)]
    {
        builder = builder
            // Muss zuerst stehen: ein zweiter Start öffnet sonst dieselbe Datenbank ein zweites Mal
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| show_main(app)))
            .plugin(tauri_plugin_process::init())
            .plugin(tauri_plugin_autostart::init(
                tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                Some(vec![background::HIDDEN_ARG]),
            ));
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
            // Ohne gelungene Sicherung wird nicht migriert; bei jedem Fehler erklärt ein Fenster, was zu tun ist.
            let conn = match startup::open_and_migrate(&dir.join(db::DB_FILE), &secrets) {
                Ok(conn) => conn,
                Err(e) => {
                    for (_, window) in app.webview_windows() {
                        let _ = window.hide();
                    }
                    startup_dialog::show_error(&startup_text::text("start.title", &[]), &e.message());
                    // Kein Fehler zurückgeben (das wäre ein Absturz mit Panik), sondern sauber beenden; die Datei bleibt unverändert.
                    app.handle().exit(1);
                    return Ok(());
                }
            };
            app.manage(AppState { db: Mutex::new(conn) });
            tasks::background::start(app.handle().clone());
            #[cfg(desktop)]
            if background::launched_hidden(&std::env::args().collect::<Vec<_>>()) {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.hide();
                }
            }

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
            commands_tasks::task_alarms_get,
            commands_tasks::task_alarms_set,
            commands_tasks::task_section_list,
            commands_tasks::task_section_create,
            commands_tasks::task_section_rename,
            commands_tasks::task_section_reorder,
            commands_tasks::task_section_delete,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Start von Daedyx Tasks");
}
