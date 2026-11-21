//! Sicherungskopie der Datenbank vor einer Migration (der Aufgaben-Kern hebt das Schema an).
use daedyx_tasks_core::SCHEMA_VERSION;
use rusqlite::{Connection, OptionalExtension};
use std::path::{Path, PathBuf};

const KEEP: usize = 3;

/// Zu wenig freier Platz für die Kopie (Werte in Bytes); die Datenbank wird dann nicht angefasst.
#[derive(Debug)]
pub struct NotEnoughSpace {
    pub needed: u64,
    pub free: u64,
}

impl std::fmt::Display for NotEnoughSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "zu wenig freier Platz ({} von {} Byte)", self.free, self.needed)
    }
}

impl std::error::Error for NotEnoughSpace {}

/// Gespeicherter Schema-Stand des Kerns. 0 = neue Datenbank (nichts zu sichern) oder nicht lesbar
/// (dann meldet die Migration den Fehler selbst und ändert nichts).
fn stored_version(conn: &Connection) -> rusqlite::Result<i64> {
    let has = |name: &str| -> rusqlite::Result<bool> {
        conn.query_row("SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1", [name], |_| Ok(())).optional().map(|r| r.is_some())
    };
    if !has("task_meta")? {
        return Ok(if has("task_lists")? { 1 } else { 0 });
    }
    let value: Option<String> = conn.query_row("SELECT value FROM task_meta WHERE key = 'schema_version'", [], |r| r.get(0)).optional()?;
    Ok(value.and_then(|v| v.parse().ok()).unwrap_or(0))
}

/// Kopiert die Datenbank nach `backups/daedyx-tasks-v<alterStand>.db`, wenn ihr Stand älter als der des Kerns,
/// aber nicht neu angelegt ist. Behält die drei neuesten Kopien. `None`, wenn nichts zu sichern war.
pub fn backup_before_migrate(conn: &Connection, db_path: &Path) -> std::io::Result<Option<PathBuf>> {
    backup_with_free_space(conn, db_path, &super::free_space::available)
}

/// Wie `backup_before_migrate`, mit austauschbarer Messung des freien Platzes (für Tests).
/// Fehlt der Platz, kommt ein `NotEnoughSpace` im `io::Error`; eine angefangene Kopie wird entfernt.
pub fn backup_with_free_space(
    conn: &Connection,
    db_path: &Path,
    free_space: &dyn Fn(&Path) -> Option<u64>,
) -> std::io::Result<Option<PathBuf>> {
    let current = stored_version(conn).map_err(std::io::Error::other)?;
    if current <= 0 || current >= SCHEMA_VERSION {
        return Ok(None);
    }
    // WAL in die Hauptdatei schreiben, damit die einzelne Datei vollständig ist.
    let busy: i64 = conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |r| r.get(0)).map_err(std::io::Error::other)?;
    if busy != 0 {
        return Err(std::io::Error::other("Datenbank war beim Sichern belegt"));
    }
    let dir = db_path.parent().unwrap_or(Path::new(".")).join("backups");
    std::fs::create_dir_all(&dir)?;
    let size = std::fs::metadata(db_path)?.len();
    let needed = size + size / 10;
    if let Some(free) = free_space(&dir) {
        if free < needed {
            return Err(std::io::Error::other(NotEnoughSpace { needed, free }));
        }
    }
    let target = dir.join(format!("daedyx-tasks-v{current}.db"));
    if let Err(e) = std::fs::copy(db_path, &target) {
        let _ = std::fs::remove_file(&target);
        return Err(e);
    }
    // Die Kopie ist fertig. Ein Fehler beim Aufräumen alter Kopien (gesperrt, schreibgeschützt) darf den Start nicht blockieren.
    let _ = prune(&dir);
    Ok(Some(target))
}

fn prune(dir: &Path) -> std::io::Result<()> {
    let mut files: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            name.starts_with("daedyx-tasks-v") && name.ends_with(".db")
        })
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    files.sort_by_key(|f| std::cmp::Reverse(f.0));
    // Ein nicht löschbarer Altbestand darf die übrigen nicht aufhalten: Fehler je Datei übergehen und weiterlöschen.
    for (_, old) in files.into_iter().skip(KEEP) {
        let _ = std::fs::remove_file(old);
    }
    Ok(())
}
