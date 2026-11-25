//! `remove_files` (T5-1): nur bekannte Dateien, fremde bleiben; Wegwerf-Ordner, `MemoryStore`.
use super::backup_tests::{dir, old_db};
use super::remove::remove_files;
use super::{backup_before_migrate, DB_FILE};
use crate::secrets::MemoryStore;

#[test]
fn bekannte_dateien_und_leere_ordner_verschwinden() {
    let base = dir("remove-all");
    let db = base.join(DB_FILE);
    let store = MemoryStore::default();
    let conn = old_db(&db, &store);
    backup_before_migrate(&conn, &db, &|_| None).unwrap().unwrap();
    drop(conn);
    std::fs::write(base.join(format!("{DB_FILE}-wal")), b"x").unwrap();
    assert!(remove_files(&base));
    assert!(!base.exists());
}

#[test]
fn fremde_dateien_bleiben_samt_ordner() {
    let base = dir("remove-foreign");
    std::fs::create_dir_all(base.join("backups")).unwrap();
    std::fs::write(base.join(DB_FILE), b"x").unwrap();
    std::fs::write(base.join("notiz.txt"), b"bleibt").unwrap();
    std::fs::write(base.join("backups").join("eigene.db"), b"bleibt").unwrap();
    assert!(remove_files(&base));
    assert!(!base.join(DB_FILE).exists());
    assert_eq!(std::fs::read(base.join("notiz.txt")).unwrap(), b"bleibt");
    assert_eq!(std::fs::read(base.join("backups").join("eigene.db")).unwrap(), b"bleibt");
}

#[test]
fn ohne_ordner_gibt_es_nichts_zu_tun() {
    let root = dir("remove-none");
    let base = root.join("gibt-es-nicht");
    assert!(remove_files(&base));
}
