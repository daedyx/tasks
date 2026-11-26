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
fn filled(name: &str) -> (daedyx_tasks_store::test_support::TestDir, MemoryStore) {
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
    assert!(remove_data(&base, None, &vault, true));
    assert!(!data_dir_path(&base).exists());
    assert_eq!(vault.get(KEY_NAME).unwrap(), None);
    assert!(base.exists(), "nur der eigene Ordner, nicht die Basis");
}

#[test]
fn produkt_laesst_nutzerdaten_immer_stehen() {
    let (base, vault) = filled("t5-cleanup-produkt");
    let db = data_dir_path(&base).join(daedyx_tasks_store::db::DB_FILE);
    let before = std::fs::read(&db).unwrap();
    assert!(remove_data(&base, None, &vault, false));
    assert_eq!(std::fs::read(&db).unwrap(), before);
    assert!(vault.get(KEY_NAME).unwrap().is_some());
}

#[test]
fn fremde_datei_bleibt_und_meldet_unvollstaendig() {
    let (base, vault) = filled("t5-cleanup-fremd");
    std::fs::write(data_dir_path(&base).join("notiz.txt"), b"bleibt").unwrap();
    assert!(!remove_data(&base, None, &vault, true));
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

/// Update U2: Zustand `updates.json`, eine liegen gebliebene Nebendatei und geladene Setups (lokal, Review U2-R
/// N-U2-4) gehören zur Vorschau und gehen mit; eine fremde Datei im Update-Ordner bleibt (dann bleibt auch der Ordner).
#[test]
fn vorschau_entfernt_auch_update_zustand_und_setups() {
    let (base, vault) = filled("u2-cleanup");
    let local = base.join("lokal");
    let dir = data_dir_path(&base);
    let updates = local_updates(&local);
    std::fs::create_dir_all(&updates).unwrap();
    std::fs::write(dir.join("updates.json"), b"{}").unwrap();
    std::fs::write(dir.join("updates.json.4711.tmp"), b"{}").unwrap();
    std::fs::write(updates.join("daedyx-tasks-vorschau-0.2.1-setup.exe"), b"x").unwrap();
    std::fs::write(
        updates.join("daedyx-tasks-vorschau-0.2.2-setup.exe.part"),
        b"x",
    )
    .unwrap();
    assert!(remove_data(&base, Some(&local), &vault, true));
    assert!(!dir.exists());
    assert!(
        !data_dir_path(&local).exists(),
        "lokaler Ordner der Vorschau weg"
    );
    assert!(local.exists(), "der lokale Basisordner selbst bleibt");

    let (base, vault) = filled("u2-cleanup-fremd");
    let local = base.join("lokal");
    let updates = local_updates(&local);
    std::fs::create_dir_all(&updates).unwrap();
    std::fs::write(updates.join("notiz.txt"), b"bleibt").unwrap();
    assert!(!remove_data(&base, Some(&local), &vault, true));
    assert_eq!(std::fs::read(updates.join("notiz.txt")).unwrap(), b"bleibt");
}

/// Der Update-Ordner im Code von `daedyx-updater` und hier sind derselbe: lokal (`%LOCALAPPDATA%\<Kennung>\updates`),
/// nicht im mitwandernden Datenordner (Review U2-R N-U2-4).
#[test]
fn update_ordner_wie_im_updater() {
    let base = tmp("u2-ordner");
    let local = base.join("lokal");
    let dir = data_dir_in(&base.join("roaming")).unwrap();
    let c = crate::update::config(&dir, &local).unwrap();
    assert_eq!(c.update_dir, local_updates(&local));
    assert!(!c.update_dir.starts_with(dir.path()));
    assert_eq!(c.data_dir, dir.path());
    assert!(
        c.lock_dir.starts_with(&local),
        "Sperre im lokalen Ordner, nicht im Datenordner"
    );
}

#[test]
fn ohne_daten_ist_nichts_zu_tun() {
    let base = tmp("t5-cleanup-leer");
    assert!(remove_data(&base, None, &MemoryStore::default(), true));
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
    assert!(!remove_data(&base, None, &vault, true));
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

fn local_updates(local: &Path) -> std::path::PathBuf {
    data_dir_path(local).join(crate::update::UPDATE_DIR)
}
