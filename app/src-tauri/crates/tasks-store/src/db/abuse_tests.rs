//! Missbrauch und Beschädigung (plan-tasks-rust T0, SICHERHEIT 5.2): Jede kaputte Datei und jeder kaputte Tresor
//! ergibt einen Fehlercode, nie eine Panik, und die Datei bleibt Byte für Byte, wie sie war.
use crate::db::backup_tests::unmeasured;
use crate::secrets::{MemoryStore, SecretError, SecretStore};
use crate::startup::{open_and_migrate, StartupCode, StartupError};
use std::path::{Path, PathBuf};

fn db_path(name: &str) -> PathBuf {
    crate::test_support::test_dir(&format!("abuse-{name}")).unwrap().join("daedyx-tasks.db")
}

/// Startet und prüft: Fehler (keine Panik), erwarteter Code, Datei unverändert, kein Ordner `backups`.
fn refused(path: &Path, store: &dyn SecretStore) -> StartupError {
    let before = std::fs::read(path).ok();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| open_and_migrate(path, store, &unmeasured)));
    let err = match result {
        Ok(Ok(_)) => panic!("kaputte Datei wurde geöffnet"),
        Ok(Err(e)) => e,
        Err(_) => panic!("Panik statt Fehlercode"),
    };
    assert_eq!(std::fs::read(path).ok(), before, "Datei verändert ({err:?})");
    assert!(!path.with_file_name("backups").exists(), "Sicherung einer kaputten Datei angelegt");
    err
}

/// Gültige, verschlüsselte Datenbank im aktuellen Stand mit erfundenen Aufgaben (mehrere Seiten groß).
fn real_db(path: &Path, store: &MemoryStore) {
    let conn = open_and_migrate(path, store, &unmeasured).unwrap();
    let list = daedyx_tasks_core::edit_lists::create_list(&conn, "Erfunden", "", 1000).unwrap();
    for i in 0..200 {
        let f = serde_json::json!({"title": format!("Erfundene Aufgabe {i}"), "notes": "x".repeat(200)});
        daedyx_tasks_core::edit_tasks::create_task(&conn, &list.id, None, f.as_object().unwrap(), 1000).unwrap();
    }
}

#[test]
fn a_plain_sqlite_file_is_not_sqlcipher_and_stays_unchanged() {
    let path = db_path("plain");
    {
        let plain = rusqlite::Connection::open(&path).unwrap();
        plain.execute_batch("CREATE TABLE t (x TEXT); INSERT INTO t VALUES ('erfunden');").unwrap();
    }
    assert!(std::fs::read(&path).unwrap().starts_with(b"SQLite format 3"));
    let err = refused(&path, &MemoryStore::default());
    assert_eq!(err.code(), StartupCode::WrongKey, "{err:?}");
}

#[test]
fn an_empty_file_is_refused_not_turned_into_a_new_database() {
    let path = db_path("empty");
    std::fs::write(&path, b"").unwrap();
    let err = refused(&path, &MemoryStore::default());
    assert_eq!(err.code(), StartupCode::Other, "{err:?}");
}

#[test]
fn a_truncated_database_is_refused_and_stays_unchanged() {
    for (name, keep) in [("half", None), ("page", Some(4096 * 2 + 100)), ("header", Some(100))] {
        let path = db_path(&format!("cut-{name}"));
        let store = MemoryStore::default();
        real_db(&path, &store);
        let bytes = std::fs::read(&path).unwrap();
        let keep = keep.unwrap_or(bytes.len() / 2);
        assert!(keep < bytes.len(), "Testdatei zu klein ({} Bytes)", bytes.len());
        std::fs::write(&path, &bytes[..keep]).unwrap();
        let err = refused(&path, &store);
        assert!(matches!(err.code(), StartupCode::Other | StartupCode::WrongKey), "{name}: {err:?}");
    }
}

#[test]
fn a_broken_key_in_the_vault_is_refused_and_kept() {
    let path = db_path("badkey");
    let store = MemoryStore::default();
    real_db(&path, &store);
    for bad in ["kaputt", "", &"z".repeat(64), &"a".repeat(65), "ä".repeat(32).as_str()] {
        store.set(crate::db::KEY_NAME, bad).unwrap();
        let err = refused(&path, &store);
        assert_eq!(err.code(), StartupCode::Other, "{err:?}");
        assert_eq!(store.get(crate::db::KEY_NAME).unwrap().as_deref(), Some(bad), "Tresor-Wert überschrieben");
    }
}

struct UnreachableVault;

impl SecretStore for UnreachableVault {
    fn get(&self, _: &str) -> Result<Option<String>, SecretError> {
        Err(SecretError::Store("nicht erreichbar".into()))
    }
    fn set(&self, _: &str, _: &str) -> Result<(), SecretError> {
        Err(SecretError::Store("nicht erreichbar".into()))
    }
    fn delete(&self, _: &str) -> Result<(), SecretError> {
        Err(SecretError::Store("nicht erreichbar".into()))
    }
}

#[test]
fn an_unreachable_vault_is_refused_and_creates_no_file() {
    let path = db_path("novault");
    let err = refused(&path, &UnreachableVault);
    assert_eq!(err.code(), StartupCode::Other, "{err:?}");
    assert!(!path.exists(), "ohne Schlüssel darf keine Datenbank entstehen");
}

#[test]
fn the_error_text_names_no_key_and_no_path() {
    let path = db_path("text");
    let store = MemoryStore::default();
    real_db(&path, &store);
    let key = store.get(crate::db::KEY_NAME).unwrap().unwrap();
    std::fs::write(&path, vec![7u8; 4096]).unwrap();
    let text = format!("{} / {:?}", refused(&path, &store), refused(&path, &store));
    assert!(!text.contains(&key), "Schlüssel im Fehlertext");
    assert!(!text.contains("daedyx-tasks-store-"), "Pfad im Fehlertext: {text}");
}
