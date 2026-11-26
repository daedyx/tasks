//! `--cleanup` für Daten und Tresor (Master-Entscheid T5-1). Nur die **Vorschau** entfernt ihren Datenordner
//! (`<Basis>\<Kennung aus ids.rs>`) und ihren Tresor-Eintrag `db-key`; die Produkt-Fassung lässt Nutzerdaten immer
//! stehen. Gelöscht werden nur bekannte Dateien (`daedyx_tasks_store::db::remove_files`, dazu seit U2 der
//! Update-Zustand `updates.json` und geladene Setups in `updates\`), nie über eine Junction. Der Schlüssel geht erst,
//! wenn die Datenbank-Datei weg ist, damit nie eine Datei ohne ihren Schlüssel übrig bleibt.
use crate::ids::APP_ID;
use crate::update::UPDATE_DIR;
use daedyx_tasks_store::db::{KEY_NAME, remove_files};
use daedyx_tasks_store::secrets::SecretStore;
use daedyx_updater::daedyx_update::download::clean_leftovers;
use daedyx_updater::daedyx_update::state::STATE_FILE;
use std::path::Path;

fn is_link(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
}

/// Update-Zustand und geladene Setups (U2); fremde Dateien bleiben, Verknüpfungen werden nie betreten.
fn remove_update_files(dir: &Path) {
    if is_link(dir) {
        return;
    }
    let _ = std::fs::remove_file(dir.join(STATE_FILE));
    // Nebendateien eines abgebrochenen Speicherns (`updates.json.<pid>.tmp`).
    if let Ok(list) = std::fs::read_dir(dir) {
        let tmp = format!("{STATE_FILE}.");
        for entry in list.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with(&tmp)
                && name.ends_with(".tmp")
                && entry.file_type().is_ok_and(|t| t.is_file())
            {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
    let updates = dir.join(UPDATE_DIR);
    if !is_link(&updates) {
        let _ = clean_leftovers(&updates);
        let _ = std::fs::remove_dir(&updates);
    }
}

/// `true`, wenn nichts von den Daten übrig ist (oder es eine Produkt-Fassung ist, die nichts entfernt).
pub fn remove_data(base: &Path, secrets: &dyn SecretStore, preview: bool) -> bool {
    if !preview {
        return true;
    }
    let dir = base.join(APP_ID);
    remove_update_files(&dir);
    if !remove_files(&dir) {
        return false;
    }
    secrets.delete(KEY_NAME).is_ok() && !dir.exists()
}
