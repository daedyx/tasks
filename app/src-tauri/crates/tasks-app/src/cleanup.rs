//! `--cleanup` für Daten und Tresor (Master-Entscheid T5-1). Nur die **Vorschau** entfernt ihren Datenordner
//! (`<Basis>\<Kennung aus ids.rs>`) und ihren Tresor-Eintrag `db-key`; die Produkt-Fassung lässt Nutzerdaten immer
//! stehen. Gelöscht werden nur bekannte Dateien (`daedyx_tasks_store::db::remove_files`), nie über eine Junction.
//! Der Schlüssel geht erst, wenn die Datenbank-Datei weg ist, damit nie eine Datei ohne ihren Schlüssel übrig bleibt.
use crate::ids::APP_ID;
use daedyx_tasks_store::db::{KEY_NAME, remove_files};
use daedyx_tasks_store::secrets::SecretStore;
use std::path::Path;

/// `true`, wenn nichts von den Daten übrig ist (oder es eine Produkt-Fassung ist, die nichts entfernt).
pub fn remove_data(base: &Path, secrets: &dyn SecretStore, preview: bool) -> bool {
    if !preview {
        return true;
    }
    let dir = base.join(APP_ID);
    if !remove_files(&dir) {
        return false;
    }
    secrets.delete(KEY_NAME).is_ok() && !dir.exists()
}
