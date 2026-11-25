//! Datenordner der App (Plan 4.2, Tasks T5). Ein [`DataDir`] entsteht nur hier: Basisordner plus Kennung aus `ids.rs`.
//! Ohne Merkmal `produkt` ist das `%APPDATA%\<Kennung der Vorschau>`; den Ordner der echten App kann dieses Programm
//! also gar nicht benennen. Gehören Kennung und Tresor nicht zur selben Fassung, gibt es keinen Ordner.
use crate::ids::{self, APP_ID};
use daedyx_tasks_store::db::DB_FILE;
use std::path::{Path, PathBuf};

/// Ordner, in dem die Datenbank der App liegt (`daedyx-tasks.db`, `-wal`, `-shm`, `backups\`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataDir(PathBuf);

impl DataDir {
    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn db_path(&self) -> PathBuf {
        self.0.join(DB_FILE)
    }
}

/// Datenordner unter `base`, benannt nach der Kennung dieser Fassung, und angelegt. `None`, wenn die Kennungen nicht
/// zusammenpassen oder der Ordner sich nicht anlegen lässt. Tests geben einen Wegwerf-Ordner als `base`.
pub fn data_dir_in(base: &Path) -> Option<DataDir> {
    if !ids::consistent() || base.as_os_str().is_empty() {
        return None;
    }
    let dir = base.join(APP_ID);
    std::fs::create_dir_all(&dir).ok()?;
    Some(DataDir(dir))
}

/// Datenordner der App unter `%APPDATA%` (macOS: `~/Library/Application Support`).
pub fn data_dir() -> Option<DataDir> {
    let base = if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| {
            ids::MAC_DATA
                .iter()
                .fold(PathBuf::from(h), |p, part| p.join(part))
        })
    } else {
        std::env::var_os("APPDATA").map(PathBuf::from)
    }?;
    data_dir_in(&base)
}
