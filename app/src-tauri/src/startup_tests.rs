use crate::db::backup_tests::{dir, old_db};
use crate::db::{self, backup_with_free_space};
use crate::secrets::MemoryStore;
use crate::startup::{StartupError, open_and_migrate};
use std::path::Path;

fn stored(path: &Path, store: &MemoryStore) -> String {
    db::open(path, store).unwrap().query_row("SELECT value FROM task_meta WHERE key = 'schema_version'", [], |r| r.get(0)).unwrap()
}

#[test]
fn a_failed_backup_stops_the_start_and_leaves_the_database_untouched() {
    let (d, store) = (dir("s-fail"), MemoryStore::default());
    let path = d.join("daedyx-tasks.db");
    drop(old_db(&path, &store));
    // „backups“ ist eine Datei statt eines Ordners: die Sicherung kann nicht angelegt werden
    std::fs::write(d.join("backups"), b"blockiert").unwrap();
    let err = open_and_migrate(&path, &store).unwrap_err();
    assert!(matches!(err, StartupError::BackupFailed(_)), "{err:?}");
    assert_eq!(stored(&path, &store), "2", "nicht migriert");
    assert!(err.message().contains("Sicherung"), "{}", err.message());
}

#[test]
fn without_free_space_no_copy_is_started_and_the_message_names_the_space() {
    let (d, store) = (dir("s-space"), MemoryStore::default());
    let path = d.join("daedyx-tasks.db");
    let conn = old_db(&path, &store);
    let e = backup_with_free_space(&conn, &path, &|_| Some(10)).unwrap_err();
    let space = e.get_ref().unwrap().downcast_ref::<db::NotEnoughSpace>().expect("NotEnoughSpace");
    assert_eq!(space.free, 10);
    assert_eq!(std::fs::read_dir(d.join("backups")).map(|r| r.count()).unwrap_or(0), 0, "keine halbe Kopie");
    let msg = StartupError::NoSpace(db::NotEnoughSpace { needed: 5 * 1024 * 1024, free: 1024 * 1024 }).message();
    assert!(msg.contains("6 MB") && msg.contains("1 MB"), "{msg}");
}

#[test]
fn a_working_backup_lets_the_migration_run() {
    let (d, store) = (dir("s-ok"), MemoryStore::default());
    let path = d.join("daedyx-tasks.db");
    drop(old_db(&path, &store));
    open_and_migrate(&path, &store).unwrap();
    assert_eq!(stored(&path, &store), daedyx_tasks_core::SCHEMA_VERSION.to_string());
    assert!(d.join("backups").join("daedyx-tasks-v2.db").exists());
}

#[test]
fn a_fresh_database_needs_no_backup_even_if_the_backup_folder_is_blocked() {
    let (d, store) = (dir("s-fresh"), MemoryStore::default());
    std::fs::write(d.join("backups"), b"blockiert").unwrap();
    open_and_migrate(&d.join("daedyx-tasks.db"), &store).unwrap();
}

#[test]
fn a_database_from_a_newer_version_gives_a_plain_message_and_stays_unchanged() {
    let (d, store) = (dir("s-toonew"), MemoryStore::default());
    let path = d.join("daedyx-tasks.db");
    let conn = old_db(&path, &store);
    conn.execute("UPDATE task_meta SET value = '99' WHERE key = 'schema_version'", []).unwrap();
    drop(conn);
    let err = open_and_migrate(&path, &store).unwrap_err();
    assert!(err.message().contains("neueren Version"), "{} / {err:?}", err.message());
    assert!(!err.message().contains("99"), "keine technischen Zahlen im Dialog");
    assert_eq!(stored(&path, &store), "99");
}

#[test]
fn a_broken_database_file_gives_a_plain_message_and_stays_unchanged() {
    let (d, store) = (dir("s-broken"), MemoryStore::default());
    let path = d.join("daedyx-tasks.db");
    std::fs::write(&path, vec![7u8; 4096]).unwrap();
    let err = open_and_migrate(&path, &store).unwrap_err();
    assert!(matches!(err, StartupError::Open(db::DbError::WrongKey)), "{err:?}");
    assert!(err.message().contains("Schlüssel"), "{}", err.message());
    assert_eq!(std::fs::read(&path).unwrap(), vec![7u8; 4096]);
}

#[test]
fn every_startup_text_exists_in_the_language_file() {
    for key in ["title", "error.backup_failed", "error.no_space", "error.too_new", "error.wrong_key", "error.generic"] {
        let full = format!("start.{key}");
        assert_ne!(crate::startup_text::text(&full, &[]), full, "Text fehlt: {full}");
    }
}
