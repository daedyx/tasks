use crate::db::{self, backup_before_migrate};
use crate::secrets::MemoryStore;
use daedyx_tasks_core::schema::{STAND_1, STAND_2};
use std::path::{Path, PathBuf};

pub fn dir(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("daedyx-tasks-backup-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

/// Eine echte Datenbank im alten Stand 2 (ohne Farben, Verweise, Erinnerungsliste).
pub fn old_db(path: &Path, store: &MemoryStore) -> rusqlite::Connection {
    let conn = db::open(path, store).unwrap();
    conn.execute_batch(STAND_1).unwrap();
    conn.execute_batch(STAND_2).unwrap();
    conn.execute("INSERT INTO task_meta (key, value) VALUES ('schema_version', '2')", []).unwrap();
    conn
}

#[test]
fn a_database_behind_the_latest_stand_is_copied_first() {
    let (d, store) = (dir("copy"), MemoryStore::default());
    let path = d.join("daedyx-tasks.db");
    let conn = old_db(&path, &store);
    let saved = backup_before_migrate(&conn, &path).unwrap().expect("Kopie erwartet");
    assert_eq!(saved.file_name().unwrap(), "daedyx-tasks-v2.db");
    assert_eq!(saved.parent().unwrap(), d.join("backups"));
    // Die Kopie ist mit demselben Schlüssel lesbar und hat noch den alten Stand.
    let copy = db::open(&saved, &store).unwrap();
    let v: String = copy.query_row("SELECT value FROM task_meta WHERE key = 'schema_version'", [], |r| r.get(0)).unwrap();
    assert_eq!(v, "2");
}

#[test]
fn nothing_is_copied_for_a_fresh_or_current_database() {
    let (d, store) = (dir("none"), MemoryStore::default());
    let path = d.join("daedyx-tasks.db");
    let fresh = db::open(&path, &store).unwrap();
    assert!(backup_before_migrate(&fresh, &path).unwrap().is_none());
    daedyx_tasks_core::migrate(&fresh).unwrap();
    assert!(backup_before_migrate(&fresh, &path).unwrap().is_none());
    assert!(!d.join("backups").exists());
}

#[test]
fn only_the_three_newest_backups_stay() {
    let (d, store) = (dir("keep"), MemoryStore::default());
    let b = d.join("backups");
    std::fs::create_dir_all(&b).unwrap();
    for v in [10, 11, 12, 13] {
        std::thread::sleep(std::time::Duration::from_millis(30));
        std::fs::write(b.join(format!("daedyx-tasks-v{v}.db")), b"x").unwrap();
    }
    let path = d.join("daedyx-tasks.db");
    let conn = old_db(&path, &store);
    backup_before_migrate(&conn, &path).unwrap();
    let names: Vec<_> = std::fs::read_dir(&b).unwrap().map(|e| e.unwrap().file_name()).collect();
    assert_eq!(names.len(), 3, "{names:?}");
    assert!(b.join("daedyx-tasks-v2.db").exists(), "die neue Kopie bleibt");
}

#[test]
fn foreign_files_in_the_backup_folder_are_never_deleted() {
    let (d, store) = (dir("foreign"), MemoryStore::default());
    let b = d.join("backups");
    std::fs::create_dir_all(&b).unwrap();
    for name in ["notizen.txt", "daedyx-desk-v1.db"] {
        std::fs::write(b.join(name), b"x").unwrap();
    }
    let path = d.join("daedyx-tasks.db");
    let conn = old_db(&path, &store);
    backup_before_migrate(&conn, &path).unwrap();
    assert!(b.join("notizen.txt").exists() && b.join("daedyx-desk-v1.db").exists());
}

#[test]
fn an_old_backup_that_cannot_be_deleted_does_not_stop_the_start() {
    let (d, store) = (dir("stuck"), MemoryStore::default());
    let b = d.join("backups");
    // Ein Ordner mit Inhalt unter dem Namen einer alten Kopie lässt sich nicht mit remove_file löschen.
    let stuck = b.join("daedyx-tasks-v9.db");
    std::fs::create_dir_all(&stuck).unwrap();
    std::fs::write(stuck.join("x"), b"x").unwrap();
    for v in [10, 11, 12] {
        std::thread::sleep(std::time::Duration::from_millis(30));
        std::fs::write(b.join(format!("daedyx-tasks-v{v}.db")), b"x").unwrap();
    }
    std::thread::sleep(std::time::Duration::from_millis(30));
    let path = d.join("daedyx-tasks.db");
    let conn = old_db(&path, &store);
    let saved = backup_before_migrate(&conn, &path).unwrap().expect("Kopie erwartet");
    assert!(saved.exists(), "die neue Kopie ist angelegt");
    assert!(stuck.exists(), "die nicht löschbare Altlast bleibt, ohne zu stören");
}

#[test]
fn a_database_from_a_newer_version_is_not_copied() {
    let (d, store) = (dir("toonew"), MemoryStore::default());
    let path = d.join("daedyx-tasks.db");
    let conn = old_db(&path, &store);
    conn.execute("UPDATE task_meta SET value = '99' WHERE key = 'schema_version'", []).unwrap();
    assert!(backup_before_migrate(&conn, &path).unwrap().is_none());
}
