//! Entfernt die Dateien einer Datenbank aus ihrem Ordner (Master-Entscheid T5-1, `--cleanup` der Vorschau). Nur
//! bekannte Namen: die Datenbank mit `-wal`/`-shm` und in `backups` die Sicherungen `daedyx-tasks-v<Stand>.db`.
//! Fremde Dateien bleiben, Ordner verschwinden nur, wenn sie danach leer sind. Verknüpfungen und Junctions werden
//! nie betreten.
use super::DB_FILE;
use std::path::Path;

fn is_link(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
}

fn remove_if_there(path: &Path) {
    if std::fs::symlink_metadata(path).is_ok_and(|m| m.is_file()) {
        let _ = std::fs::remove_file(path);
    }
}

/// `true`, wenn danach keine Datenbank-Datei mehr im Ordner liegt (auch wenn es nie eine gab).
pub fn remove_files(dir: &Path) -> bool {
    if is_link(dir) {
        return false;
    }
    for suffix in ["", "-wal", "-shm"] {
        remove_if_there(&dir.join(format!("{DB_FILE}{suffix}")));
    }
    let backups = dir.join("backups");
    if !is_link(&backups)
        && let Ok(entries) = std::fs::read_dir(&backups)
    {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("daedyx-tasks-v") && name.ends_with(".db") {
                remove_if_there(&entry.path());
            }
        }
        let _ = std::fs::remove_dir(&backups);
    }
    let _ = std::fs::remove_dir(dir);
    !dir.join(DB_FILE).exists()
}
