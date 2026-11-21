//! Datenbank beim Start öffnen, sichern und migrieren. Ohne Sicherung gibt es keine Migration.
use crate::db::{self, DbError, NotEnoughSpace};
use crate::secrets::SecretStore;
use crate::startup_text::text;
use daedyx_tasks_core::MigrateError;
use rusqlite::Connection;
use std::path::Path;

#[derive(Debug)]
pub enum StartupError {
    Open(DbError),
    NoSpace(NotEnoughSpace),
    BackupFailed(std::io::Error),
    Migrate(MigrateError),
}

const MB: u64 = 1024 * 1024;

impl StartupError {
    /// Satz in einfacher Sprache für den Dialog; technische Angaben stehen nur beim unbekannten Fehler.
    pub fn message(&self) -> String {
        match self {
            Self::Open(DbError::WrongKey) => text("start.error.wrong_key", &[]),
            Self::Migrate(MigrateError::TooNew { .. }) => text("start.error.too_new", &[]),
            Self::NoSpace(n) => {
                let (need, free) = ((n.needed / MB + 1).to_string(), (n.free / MB).to_string());
                text("start.error.no_space", &[("needed", &need), ("free", &free)])
            }
            Self::BackupFailed(_) => text("start.error.backup_failed", &[]),
            Self::Open(e) => text("start.error.generic", &[("detail", &e.to_string())]),
            Self::Migrate(e) => text("start.error.generic", &[("detail", &e.to_string())]),
        }
    }
}

pub fn open_and_migrate(db_path: &Path, secrets: &dyn SecretStore) -> Result<Connection, StartupError> {
    let conn = db::open(db_path, secrets).map_err(StartupError::Open)?;
    // Eine neue, leere Datenbank wird nicht gesichert; hat sie Daten und die Sicherung scheitert, wird NICHT migriert.
    if let Err(e) = db::backup_before_migrate(&conn, db_path) {
        return Err(match e.get_ref().and_then(|i| i.downcast_ref::<NotEnoughSpace>()) {
            Some(n) => StartupError::NoSpace(NotEnoughSpace { needed: n.needed, free: n.free }),
            None => StartupError::BackupFailed(e),
        });
    }
    daedyx_tasks_core::migrate(&conn).map_err(StartupError::Migrate)?;
    Ok(conn)
}
