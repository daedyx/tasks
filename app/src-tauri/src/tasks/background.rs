//! Takt für Erinnerungen: in festen Abständen einen Durchgang aus `daedyx-tasks-store` laufen lassen und Fälliges als
//! `tasks://reminder` melden. Der Durchgang selbst ist eine reine Funktion (`reminders::tick_once`).
use daedyx_tasks_store::reminders::{FIRST_TICK_DELAY, TICK_INTERVAL, local_zone, tick_once, unix_now};
use tauri::{AppHandle, Emitter, Manager};

pub fn start(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(FIRST_TICK_DELAY).await;
        loop {
            let a = app.clone();
            let _ = tauri::async_runtime::spawn_blocking(move || {
                let state = a.state::<crate::AppState>();
                tick_once(&state.db, unix_now(), &local_zone(), |r| {
                    let _ = a.emit("tasks://reminder", r);
                });
            })
            .await;
            tokio::time::sleep(TICK_INTERVAL).await;
        }
    });
}
