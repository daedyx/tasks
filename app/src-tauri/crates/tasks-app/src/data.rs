//! Echte Datenbank der Vorschau (Tasks T5): eigene SQLCipher-Datei im Datenordner der Vorschau (`paths::DataDir`),
//! Schlüssel im Tresor-Dienst der Vorschau (`ids::vault()`), Sicherung vor jeder Migration, Startfehler als Code. Beim
//! allerersten Start (es gab noch keine Datei) füllt der Generator erfundene Aufgaben ein (Entscheid T-F2: die
//! Vorschau zeigt nur erfundene Daten; sie liest weder den echten Ordner noch den echten Tresor-Eintrag).
use crate::paths::DataDir;
use crate::reminders::{SharedConn, SharedDb};
use crate::seed::DEMO;
use chrono_tz::Tz;
use daedyx_tasks_store::db::FreeSpace;
use daedyx_tasks_store::secrets::SecretStore;
use daedyx_tasks_store::startup::{StartupError, open_and_migrate};
use daedyx_tasks_ui::{Clock, TaskStore};
use std::sync::{Arc, Mutex};

/// Geöffnete Datenbank für Oberfläche, Takt und Einstellungen.
pub struct Opened {
    pub db: SharedConn,
    /// Die Datei wurde bei diesem Start angelegt und mit erfundenen Aufgaben gefüllt.
    pub created: bool,
}

/// Öffnet (oder legt an) die Datenbank in `dir`. Fehler kommen als `StartupError` (Fenster mit Code); die Datei
/// bleibt dann unverändert. Scheitert nur das Befüllen einer neuen Datei, startet die App leer.
pub fn open(
    dir: &DataDir,
    secrets: &dyn SecretStore,
    free_space: FreeSpace<'_>,
    tz: Tz,
    clock: Clock,
) -> Result<Opened, StartupError> {
    let path = dir.db_path();
    let created = !path.exists();
    let conn = open_and_migrate(&path, secrets, free_space)?;
    let db = Arc::new(Mutex::new(conn));
    if created {
        let store = TaskStore::on(Box::new(SharedDb(Arc::clone(&db))), tz, clock);
        let _ = daedyx_tasks_ui::seed::fill(&store, DEMO);
    }
    Ok(Opened { db, created })
}

/// Oberfläche auf der geöffneten Datenbank.
pub fn store(opened: &Opened, tz: Tz, clock: Clock) -> TaskStore {
    TaskStore::on(Box::new(SharedDb(Arc::clone(&opened.db))), tz, clock)
}
