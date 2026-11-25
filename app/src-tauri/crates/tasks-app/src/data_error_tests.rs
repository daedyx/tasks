//! Startfehler mit echten Codes (Plan 4.5, Tasks T5): zu neue Datenbank, falscher Schlüssel, kaputte Datei, leere
//! Datei, zu wenig Platz, Sicherung scheitert. Jedes Mal der richtige Code, ein Fenstertext ohne Pfad und die Datei
//! unverändert (Prüfsumme). Wegwerf-Ordner, `MemoryStore`, erfundene Daten.
use crate::data;
use crate::paths::{DataDir, data_dir_in};
use crate::startfehler::message;
use crate::test_support::{allowed_files_only, checksum, clock, tmp};
use chrono_tz::Europe::Berlin;
use daedyx_tasks_store::db;
use daedyx_tasks_store::secrets::MemoryStore;
use daedyx_tasks_store::startup::StartupCode;
use daedyx_ui::i18n::Language;
use std::path::Path;

fn unmeasured(_: &Path) -> Option<u64> {
    None
}

/// Öffnet und erwartet einen Fehler; prüft Code, Fenstertext und dass die Datei gleich blieb.
fn refused(dir: &DataDir, vault: &MemoryStore, free: &dyn Fn(&Path) -> Option<u64>) -> StartupCode {
    let before = checksum(&dir.db_path());
    let Err(e) = data::open(dir, vault, free, Berlin, clock()) else {
        panic!("Öffnen hätte scheitern müssen");
    };
    let code = e.code();
    let text = message(code, &crate::texts::texts(Language::De));
    assert!(
        text.len() > 40 && !text.contains(&*dir.path().to_string_lossy()),
        "{text}"
    );
    assert_eq!(checksum(&dir.db_path()), before, "Datei unverändert");
    code
}

fn old_db(dir: &DataDir, vault: &MemoryStore) {
    let conn = db::open(&dir.db_path(), vault).unwrap();
    conn.execute_batch(daedyx_tasks_core::schema::STAND_1)
        .unwrap();
    conn.execute_batch(daedyx_tasks_core::schema::STAND_2)
        .unwrap();
    conn.execute(
        "INSERT INTO task_meta (key, value) VALUES ('schema_version', '2')",
        [],
    )
    .unwrap();
    conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE)")
        .unwrap();
}

#[test]
fn zu_neue_datenbank() {
    let dir = data_dir_in(&tmp("t5-zuneu")).unwrap();
    let vault = MemoryStore::default();
    drop(data::open(&dir, &vault, &unmeasured, Berlin, clock()).unwrap());
    {
        let conn = db::open(&dir.db_path(), &vault).unwrap();
        conn.execute(
            "UPDATE task_meta SET value = '99' WHERE key = 'schema_version'",
            [],
        )
        .unwrap();
        conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE)")
            .unwrap();
    }
    assert_eq!(refused(&dir, &vault, &unmeasured), StartupCode::TooNew);
    allowed_files_only(&dir);
}

#[test]
fn falscher_schluessel() {
    let dir = data_dir_in(&tmp("t5-schluessel")).unwrap();
    drop(data::open(&dir, &MemoryStore::default(), &unmeasured, Berlin, clock()).unwrap());
    assert_eq!(
        refused(&dir, &MemoryStore::default(), &unmeasured),
        StartupCode::WrongKey
    );
    allowed_files_only(&dir);
}

#[test]
fn kaputte_datei() {
    let dir = data_dir_in(&tmp("t5-kaputt")).unwrap();
    let junk: Vec<u8> = (0..8192u32)
        .map(|i| u8::try_from(i.wrapping_mul(2_654_435_761) >> 24).unwrap())
        .collect();
    std::fs::write(dir.db_path(), junk).unwrap();
    assert_eq!(
        refused(&dir, &MemoryStore::default(), &unmeasured),
        StartupCode::WrongKey
    );
    allowed_files_only(&dir);
}

#[test]
fn leere_datei_wird_nicht_zur_neuen_datenbank() {
    let dir = data_dir_in(&tmp("t5-leer")).unwrap();
    std::fs::write(dir.db_path(), b"").unwrap();
    assert_eq!(
        refused(&dir, &MemoryStore::default(), &unmeasured),
        StartupCode::Other
    );
    assert_eq!(std::fs::metadata(dir.db_path()).unwrap().len(), 0);
    allowed_files_only(&dir);
}

#[test]
fn zu_wenig_platz_fuer_die_sicherung() {
    let dir = data_dir_in(&tmp("t5-platz")).unwrap();
    let vault = MemoryStore::default();
    old_db(&dir, &vault);
    let code = refused(&dir, &vault, &|_| Some(0));
    assert!(
        matches!(code, StartupCode::NoSpace { free_mb: 0, .. }),
        "{code:?}"
    );
    allowed_files_only(&dir);
}

#[test]
fn sicherung_scheitert_dann_keine_migration() {
    let dir = data_dir_in(&tmp("t5-sicherung")).unwrap();
    let vault = MemoryStore::default();
    old_db(&dir, &vault);
    // Eine Datei namens `backups` versperrt den Ordner für die Sicherung.
    std::fs::write(dir.path().join("backups"), b"x").unwrap();
    assert_eq!(
        refused(&dir, &vault, &unmeasured),
        StartupCode::BackupFailed
    );
    std::fs::remove_file(dir.path().join("backups")).unwrap();
    let conn = db::open(&dir.db_path(), &vault).unwrap();
    let v: String = conn
        .query_row(
            "SELECT value FROM task_meta WHERE key = 'schema_version'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(v, "2", "nicht migriert");
    drop(conn);
    allowed_files_only(&dir);
}
