//! Verschlüsselte Datenbank (SQLCipher). Schlüssel im Tresor; Schema und Versionierung gehören dem Aufgaben-Kern.
#[cfg(test)]
mod abuse_tests;
mod backup;
#[cfg(test)]
pub(crate) mod backup_tests;
mod key;
#[cfg(test)]
mod tests;

pub use backup::{backup_before_migrate, FreeSpace, NotEnoughSpace};
pub use key::KEY_NAME;

use crate::secrets::{SecretError, SecretStore};
use rusqlite::{Connection, ErrorCode};
use std::path::Path;

/// Dateiname der Datenbank im Datenordner der App (Windows `%APPDATA%\de.daedyx.tasks`, macOS `~/Library/Application Support/de.daedyx.tasks`).
pub const DB_FILE: &str = "daedyx-tasks.db";

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    /// Datei ist mit diesem Schlüssel nicht lesbar. NIE als Anlass benutzen, die Datenbank zu löschen.
    #[error("Datenbankschlüssel passt nicht zur Datenbank")]
    WrongKey,
    #[error("Der Datenbankschlüssel im Tresor ist ungültig")]
    BadKey,
    #[error("Tresor: {0}")]
    Secret(#[from] SecretError),
    #[error("Datenbank: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Aufgaben: {0}")]
    Tasks(#[from] daedyx_tasks_core::MigrateError),
    #[error("Zufallszahlen nicht verfügbar: {0}")]
    Random(String),
    /// Die Datei ist da, aber leer. SQLite machte daraus still eine neue Datenbank; leer ist aber kein neuer Start,
    /// sondern ein Hinweis auf Datenverlust (die Sicherungen liegen in `backups`).
    #[error("Datenbank-Datei ist leer")]
    EmptyFile,
}

/// Einzige Stelle im Workspace, die eine Datenbankdatei öffnet (außerhalb des Stores sperrt Clippy das, Plan 4.2).
#[allow(clippy::disallowed_methods, reason = "SICHERHEIT-AUSNAHMEN A-030: einzige Öffnungsstelle, im Store")]
pub(crate) fn connect(path: &Path) -> rusqlite::Result<Connection> {
    Connection::open(path)
}

/// Öffnet (oder erstellt) die verschlüsselte Datenbank. Der Schlüssel kommt aus dem Tresor.
/// `WrongKey` nie zum Löschen der DB verwenden; andere Fehler kommen als `Sqlite` durch.
/// Eine vorhandene, leere Datei wird nicht angefasst (`EmptyFile`).
pub fn open(path: &Path, secrets: &dyn SecretStore) -> Result<Connection, DbError> {
    if std::fs::metadata(path).is_ok_and(|m| m.is_file() && m.len() == 0) {
        return Err(DbError::EmptyFile);
    }
    let hex = key::get_or_create(secrets)?;
    let conn = connect(path)?;
    conn.pragma_update(None, "key", format!("x'{hex}'"))?;
    // Erst die Prüfabfrage zeigt, ob der Schlüssel zur Datei passt.
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |r| r.get::<_, i64>(0)).map_err(|e| match e {
        rusqlite::Error::SqliteFailure(f, _) if f.code == ErrorCode::NotADatabase => DbError::WrongKey,
        other => DbError::Sqlite(other),
    })?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    Ok(conn)
}
