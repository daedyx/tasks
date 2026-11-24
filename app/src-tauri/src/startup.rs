//! Text des Startfehler-Fensters zum Fehlercode aus `daedyx-tasks-store`. Öffnen, Sichern und Migrieren selbst
//! liegen im Store-Crate (`daedyx_tasks_store::startup`).
use crate::startup_text::text;
use daedyx_tasks_store::startup::{StartupCode, StartupError};

/// Satz in einfacher Sprache für den Dialog; technische Angaben stehen nur beim unbekannten Fehler.
pub fn message(e: &StartupError) -> String {
    match e.code() {
        StartupCode::WrongKey => text("start.error.wrong_key", &[]),
        StartupCode::TooNew => text("start.error.too_new", &[]),
        StartupCode::NoSpace { needed_mb, free_mb } => {
            text("start.error.no_space", &[("needed", &needed_mb.to_string()), ("free", &free_mb.to_string())])
        }
        StartupCode::BackupFailed => text("start.error.backup_failed", &[]),
        StartupCode::Other => text("start.error.generic", &[("detail", &e.to_string())]),
    }
}
