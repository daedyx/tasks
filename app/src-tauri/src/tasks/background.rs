//! Takt für Erinnerungen: alle 30 Sekunden nach Fälligem sehen und `tasks://reminder` melden.
use super::reminders;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

fn unix_now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

pub fn start(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_secs(4)).await;
        loop {
            let a = app.clone();
            let _ = tauri::async_runtime::spawn_blocking(move || {
                let state = a.state::<crate::AppState>();
                let tz = iana_time_zone::get_timezone().unwrap_or_else(|_| "UTC".to_string());
                if let Ok(conn) = state.db.lock() {
                    let _ = reminders::tick(&conn, unix_now(), &tz, |r| {
                        let _ = a.emit("tasks://reminder", r);
                    });
                }
            })
            .await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}
