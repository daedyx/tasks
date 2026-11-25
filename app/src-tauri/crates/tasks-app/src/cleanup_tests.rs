//! `--cleanup` (Master-Entscheid T5-1) am Wegwerf-Ordner mit `MemoryStore`: Vorschau entfernt Daten und Schlüssel,
//! Produkt lässt alles stehen, fremde Dateien bleiben.
use crate::cleanup::remove_data;
use crate::data;
use crate::paths::data_dir_in;
use crate::test_support::{clock, tmp};
use chrono_tz::Europe::Berlin;
use daedyx_tasks_store::db::KEY_NAME;
use daedyx_tasks_store::secrets::{MemoryStore, SecretStore};
use std::path::Path;

fn unmeasured(_: &Path) -> Option<u64> {
    None
}

/// Datenordner mit echter (erfundener) Datenbank und einer Sicherung.
fn filled(name: &str) -> (std::path::PathBuf, MemoryStore) {
    let base = tmp(name);
    let dir = data_dir_in(&base).unwrap();
    let vault = MemoryStore::default();
    drop(data::open(&dir, &vault, &unmeasured, Berlin, clock()).unwrap());
    std::fs::create_dir_all(dir.path().join("backups")).unwrap();
    std::fs::write(
        dir.path().join("backups").join("daedyx-tasks-v2.db"),
        b"alt",
    )
    .unwrap();
    (base, vault)
}

#[test]
fn vorschau_entfernt_ordner_und_schluessel() {
    let (base, vault) = filled("t5-cleanup");
    assert!(vault.get(KEY_NAME).unwrap().is_some());
    assert!(remove_data(&base, &vault, true));
    assert!(!data_dir_path(&base).exists());
    assert_eq!(vault.get(KEY_NAME).unwrap(), None);
    assert!(base.exists(), "nur der eigene Ordner, nicht die Basis");
}

#[test]
fn produkt_laesst_nutzerdaten_immer_stehen() {
    let (base, vault) = filled("t5-cleanup-produkt");
    let db = data_dir_path(&base).join(daedyx_tasks_store::db::DB_FILE);
    let before = std::fs::read(&db).unwrap();
    assert!(remove_data(&base, &vault, false));
    assert_eq!(std::fs::read(&db).unwrap(), before);
    assert!(vault.get(KEY_NAME).unwrap().is_some());
}

#[test]
fn fremde_datei_bleibt_und_meldet_unvollstaendig() {
    let (base, vault) = filled("t5-cleanup-fremd");
    std::fs::write(data_dir_path(&base).join("notiz.txt"), b"bleibt").unwrap();
    assert!(!remove_data(&base, &vault, true));
    assert_eq!(
        std::fs::read(data_dir_path(&base).join("notiz.txt")).unwrap(),
        b"bleibt"
    );
    assert_eq!(
        vault.get(KEY_NAME).unwrap(),
        None,
        "Datenbank ist weg, also auch ihr Schlüssel"
    );
}

#[test]
fn ohne_daten_ist_nichts_zu_tun() {
    let base = tmp("t5-cleanup-leer");
    assert!(remove_data(&base, &MemoryStore::default(), true));
    assert!(!data_dir_path(&base).exists());
}

/// Review T5, H5-2: Ist der Datenordner eine Junction (etwa auf einen anderen Datenordner), wird darin nichts
/// gelöscht und der Schlüssel bleibt. Die Junction legt `cmd mklink /J` an (über `daedyx-sys`, kein eigener Prozess).
#[cfg(windows)]
#[test]
fn junction_als_datenordner_wird_nie_betreten() {
    use daedyx_sys::{RunEnd, run_with_limit};
    let base = tmp("t5-cleanup-junction");
    let target = base.join("ziel");
    std::fs::create_dir_all(target.join("backups")).unwrap();
    std::fs::write(target.join(daedyx_tasks_store::db::DB_FILE), b"bleibt").unwrap();
    std::fs::write(target.join("backups").join("daedyx-tasks-v2.db"), b"bleibt").unwrap();
    let link = data_dir_path(&base);
    let shell = std::env::var_os("ComSpec").unwrap();
    let args = [
        "/C",
        "mklink",
        "/J",
        link.to_str().unwrap(),
        target.to_str().unwrap(),
    ];
    let made = run_with_limit(Path::new(&shell), &args, std::time::Duration::from_secs(15));
    assert_eq!(made, RunEnd::Exited(Some(0)));
    let vault = MemoryStore::default();
    vault.set(KEY_NAME, "schluessel-bleibt").unwrap();
    assert!(!remove_data(&base, &vault, true));
    assert_eq!(
        std::fs::read(target.join(daedyx_tasks_store::db::DB_FILE)).unwrap(),
        b"bleibt"
    );
    assert!(target.join("backups").join("daedyx-tasks-v2.db").exists());
    assert!(vault.get(KEY_NAME).unwrap().is_some());
    let _ = std::fs::remove_dir(&link);
}

fn data_dir_path(base: &Path) -> std::path::PathBuf {
    base.join(crate::ids::APP_ID)
}
