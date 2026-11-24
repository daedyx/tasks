//! Datenbank beim Start öffnen, sichern und migrieren. Ohne Sicherung gibt es keine Migration.
//! Fehler kommen als `StartupError` mit einem `StartupCode` heraus; die Texte macht die Oberfläche (Sprachdatei).
use crate::db::{self, DbError, FreeSpace, NotEnoughSpace};
use crate::secrets::SecretStore;
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

/// Was die Oberfläche der Person sagen soll; ohne Text, ohne Pfad, ohne Inhalt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupCode {
    /// Die Datei lässt sich mit dem Schlüssel aus dem Tresor nicht lesen (auch: keine SQLCipher-Datei).
    WrongKey,
    /// Die Datenbank stammt aus einer neueren Version.
    TooNew,
    /// Zu wenig Platz für die Sicherung, in MB (`needed_mb` aufgerundet).
    NoSpace { needed_mb: u64, free_mb: u64 },
    /// Die Sicherung vor der Migration ist gescheitert; nichts wurde migriert.
    BackupFailed,
    /// Alles andere (Tresor nicht erreichbar oder Wert kaputt, Datei beschädigt, Migration gescheitert).
    Other,
}

const MB: u64 = 1024 * 1024;

impl StartupError {
    pub fn code(&self) -> StartupCode {
        match self {
            Self::Open(DbError::WrongKey) => StartupCode::WrongKey,
            Self::Migrate(MigrateError::TooNew { .. }) => StartupCode::TooNew,
            Self::NoSpace(n) => {
                StartupCode::NoSpace { needed_mb: (n.needed / MB).saturating_add(1), free_mb: n.free / MB }
            }
            Self::BackupFailed(_) => StartupCode::BackupFailed,
            Self::Open(_) | Self::Migrate(_) => StartupCode::Other,
        }
    }
}

/// Technische Angabe für den Fall `Other` (Fehlerart, nie Schlüssel oder Inhalte).
impl std::fmt::Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open(e) => write!(f, "{e}"),
            Self::NoSpace(n) => write!(f, "{n}"),
            Self::BackupFailed(e) => write!(f, "{e}"),
            Self::Migrate(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for StartupError {}

/// Öffnet die Datenbank, sichert sie bei Bedarf und migriert sie auf den Stand des Kerns.
pub fn open_and_migrate(
    db_path: &Path,
    secrets: &dyn SecretStore,
    free_space: FreeSpace<'_>,
) -> Result<Connection, StartupError> {
    let conn = db::open(db_path, secrets).map_err(StartupError::Open)?;
    // Eine neue, leere Datenbank wird nicht gesichert; hat sie Daten und die Sicherung scheitert, wird NICHT migriert.
    if let Err(e) = db::backup_before_migrate(&conn, db_path, free_space) {
        return Err(match e.get_ref().and_then(|i| i.downcast_ref::<NotEnoughSpace>()) {
            Some(n) => StartupError::NoSpace(NotEnoughSpace { needed: n.needed, free: n.free }),
            None => StartupError::BackupFailed(e),
        });
    }
    daedyx_tasks_core::migrate(&conn).map_err(StartupError::Migrate)?;
    Ok(conn)
}
