use super::*;
use crate::db::backup_tests::unmeasured;
use crate::secrets::{MemoryStore, SecretStore};
use crate::test_support::TestFile;
use std::path::Path;

const MARKER: &str = "GEHEIMER-TESTWERT-4711";

fn temp_db(name: &str) -> TestFile {
    crate::test_support::test_dir(&format!("db-{name}")).unwrap().file("test.db")
}

fn write_marker(path: &Path, store: &MemoryStore) {
    let conn = open(path, store).unwrap();
    conn.execute_batch("CREATE TABLE t (x TEXT)").unwrap();
    conn.execute("INSERT INTO t (x) VALUES (?1)", [MARKER]).unwrap();
}

#[test]
fn open_creates_encrypted_file() {
    let path = temp_db("encrypted");
    write_marker(&path, &MemoryStore::default());
    let bytes = std::fs::read(&path).unwrap();
    assert!(!bytes.starts_with(b"SQLite format 3"));
    assert!(bytes.len() > 1000, "Datei ist verdächtig klein");
    assert!(!bytes.windows(MARKER.len()).any(|w| w == MARKER.as_bytes()));
}

#[test]
fn stored_key_opens_file_with_raw_pragma() {
    let path = temp_db("rawkey");
    let store = MemoryStore::default();
    write_marker(&path, &store);
    let hex = store.get("db-key").unwrap().unwrap();
    let raw = connect(&path).unwrap();
    raw.pragma_update(None, "key", format!("x'{hex}'")).unwrap();
    let v: String = raw.query_row("SELECT x FROM t", [], |r| r.get(0)).unwrap();
    assert_eq!(v, MARKER);
}

#[test]
fn random_bytes_file_is_wrong_key() {
    let path = temp_db("random");
    let mut junk = vec![0u8; 8192];
    getrandom::fill(&mut junk).unwrap();
    std::fs::write(&path, junk).unwrap();
    assert!(matches!(open(&path, &MemoryStore::default()), Err(DbError::WrongKey)));
}

#[test]
fn malformed_stored_key_is_rejected_and_kept() {
    let path = temp_db("badkey");
    for bad in ["zu-kurz", &"g".repeat(64), &"a".repeat(63)] {
        let store = MemoryStore::default();
        store.set("db-key", bad).unwrap();
        assert!(matches!(open(&path, &store), Err(DbError::BadKey)));
        assert_eq!(store.get("db-key").unwrap().as_deref(), Some(bad));
    }
}

#[test]
fn file_unreadable_without_key() {
    let path = temp_db("nokey");
    write_marker(&path, &MemoryStore::default());
    let raw = connect(&path).unwrap();
    let res: Result<i64, _> = raw.query_row("SELECT count(*) FROM sqlite_master", [], |r| r.get(0));
    assert!(res.is_err());
}

#[test]
fn first_start_creates_the_task_schema_and_a_second_start_keeps_the_data() {
    let path = temp_db("migrate");
    let store = MemoryStore::default();
    {
        let conn = crate::startup::open_and_migrate(&path, &store, &unmeasured).unwrap();
        let lists = daedyx_tasks_core::store::lists(&conn).unwrap();
        assert!(lists.is_empty(), "neue Datenbank beginnt leer");
        daedyx_tasks_core::edit_lists::create_list(&conn, "Testliste", "blue", 1_000).unwrap();
    }
    let conn = crate::startup::open_and_migrate(&path, &store, &unmeasured).unwrap();
    let lists = daedyx_tasks_core::store::lists(&conn).unwrap();
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].name, "Testliste");
}

#[test]
fn a_database_from_a_newer_core_is_rejected_not_changed() {
    let path = temp_db("toonew");
    let store = MemoryStore::default();
    {
        let conn = crate::startup::open_and_migrate(&path, &store, &unmeasured).unwrap();
        conn.execute("UPDATE task_meta SET value = '999' WHERE key = 'schema_version'", []).unwrap();
    }
    let res = crate::startup::open_and_migrate(&path, &store, &unmeasured);
    assert!(
        matches!(res, Err(crate::startup::StartupError::Migrate(daedyx_tasks_core::MigrateError::TooNew { .. }))),
        "{res:?}"
    );
}

/// Review T5, N5-2: Der Schlüssel liegt als `Zeroizing` im Speicher (wird beim Freigeben überschrieben), hat kein
/// `Debug` nach außen und ist genau 64 Hex-Zeichen lang; der Text für das `PRAGMA` ebenso.
#[test]
fn schluessel_und_pragma_werden_im_speicher_geloescht() {
    let store = MemoryStore::default();
    let key: zeroize::Zeroizing<String> = key::get_or_create(&store).unwrap();
    assert_eq!(key.len(), 64);
    assert!(key.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    let again = key::get_or_create(&store).unwrap();
    assert_eq!(*key, *again);
    let pragma: zeroize::Zeroizing<String> = key::pragma(&key);
    assert_eq!(pragma.len(), 67);
    assert!(pragma.starts_with("x'") && pragma.ends_with('\''));
}
