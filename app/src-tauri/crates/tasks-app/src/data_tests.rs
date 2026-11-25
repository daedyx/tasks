//! Daten-Tests nach Plan 4.5 (Tasks T5, ohne Kopie-Import: Entscheid T-F2). Jeder Test arbeitet in einem eigenen
//! Wegwerf-Ordner (Datenordner = `<Ordner>\<Kennung der Vorschau>`), mit `MemoryStore` statt Tresor, nur mit
//! erfundenen Aufgaben, und prüft am Ende die Positivliste der Dateien.
use crate::data::{self, Opened};
use crate::paths::{DataDir, data_dir_in};
use crate::test_support::{allowed_files_only, checksum, tmp};
use chrono_tz::Europe::Berlin;
use daedyx_tasks_core::SCHEMA_VERSION;
use daedyx_tasks_store::db;
use daedyx_tasks_store::prefs::{self, KEYS};
use daedyx_tasks_store::secrets::MemoryStore;
use daedyx_tasks_ui::Clock;
use std::path::Path;

fn fixed() -> Clock {
    crate::test_support::clock()
}

fn unmeasured(_: &Path) -> Option<u64> {
    None
}

fn open(dir: &DataDir, vault: &MemoryStore) -> Opened {
    data::open(dir, vault, &unmeasured, Berlin, fixed()).unwrap()
}

fn count(o: &Opened, table: &str) -> i64 {
    let conn = o.db.lock().unwrap();
    conn.query_row(
        &format!("SELECT count(*) FROM {table} WHERE deleted_at IS NULL"),
        [],
        |r| r.get(0),
    )
    .unwrap()
}

fn version(o: &Opened) -> String {
    let conn = o.db.lock().unwrap();
    conn.query_row(
        "SELECT value FROM task_meta WHERE key = 'schema_version'",
        [],
        |r| r.get(0),
    )
    .unwrap()
}

#[test]
fn datenordner_traegt_die_kennung_der_vorschau() {
    let base = tmp("t5-ordner");
    let dir = data_dir_in(&base).unwrap();
    assert_eq!(dir.path(), base.join(crate::ids::APP_ID));
    assert!(crate::ids::consistent());
    assert_eq!(
        dir.db_path(),
        base.join(crate::ids::APP_ID).join(db::DB_FILE)
    );
    assert!(
        data_dir_in(Path::new("")).is_none(),
        "ohne Basis kein Ordner"
    );
}

#[test]
fn frische_datenbank_ist_verschluesselt_stand_3_mit_erfundenen_aufgaben_und_standardwerten() {
    let base = tmp("t5-frisch");
    let dir = data_dir_in(&base).unwrap();
    let vault = MemoryStore::default();
    let o = open(&dir, &vault);
    assert!(o.created);
    assert!(
        count(&o, "task_lists") >= 3 && count(&o, "tasks") > 10,
        "Generator hat erfunden"
    );
    assert_eq!(version(&o), SCHEMA_VERSION.to_string());
    {
        let conn = o.db.lock().unwrap();
        for key in KEYS {
            assert_eq!(prefs::get(&conn, key), None, "{key}: Standardwert");
        }
        assert!(prefs::board_modes(&conn).is_empty());
    }
    drop(o);
    let head = std::fs::read(dir.db_path()).unwrap();
    assert!(
        !head.starts_with(b"SQLite format 3"),
        "Datei ist verschlüsselt"
    );
    allowed_files_only(&dir);
}

#[test]
fn zweiter_start_fuellt_nicht_noch_einmal() {
    let base = tmp("t5-zweiter");
    let dir = data_dir_in(&base).unwrap();
    let vault = MemoryStore::default();
    let first = count(&open(&dir, &vault), "tasks");
    let again = open(&dir, &vault);
    assert!(!again.created);
    assert_eq!(count(&again, "tasks"), first);
    drop(again);
    allowed_files_only(&dir);
}

/// Wie die Tauri-Fassung sie hinterlässt: Stand 3 über denselben Store und Kern-Tag, mit Liste, Abschnitt, Aufgabe,
/// Unteraufgabe und zwei Erinnerungen (erfunden).
fn tauri_db(path: &Path, vault: &MemoryStore) {
    use daedyx_tasks_core::{edit_alarms, edit_lists, edit_sections, edit_tasks};
    let conn = db::open(path, vault).unwrap();
    daedyx_tasks_core::migrate(&conn).unwrap();
    let list = edit_lists::create_list(&conn, "Garten", "", 1000).unwrap();
    let section = edit_sections::create_section(&conn, &list.id, "Frühling", 1000).unwrap();
    let f = serde_json::json!({"title": "Beet umgraben", "due": "2026-12-01", "section_id": section.id});
    let task =
        edit_tasks::create_task(&conn, &list.id, None, f.as_object().unwrap(), 1000).unwrap();
    let kid = serde_json::json!({"title": "Spaten holen"});
    edit_tasks::create_task(
        &conn,
        &list.id,
        Some(&task.id),
        kid.as_object().unwrap(),
        1000,
    )
    .unwrap();
    edit_alarms::set_alarms(&conn, &task.id, &[0, 1440], 1000).unwrap();
}

#[test]
fn vorhandene_stand_3_datei_oeffnet_mit_allem_und_ohne_schemaaenderung() {
    let base = tmp("t5-tauri");
    let dir = data_dir_in(&base).unwrap();
    let vault = MemoryStore::default();
    tauri_db(&dir.db_path(), &vault);
    let o = open(&dir, &vault);
    assert!(!o.created, "vorhandene Datei, kein Generator");
    assert_eq!(
        (
            count(&o, "task_lists"),
            count(&o, "tasks"),
            count(&o, "task_sections")
        ),
        (1, 2, 1)
    );
    assert_eq!(count(&o, "task_alarms"), 2);
    assert_eq!(version(&o), SCHEMA_VERSION.to_string());
    let store = data::store(&o, Berlin, fixed());
    let lists = store.lists().unwrap();
    assert_eq!(lists[0].name, "Garten");
    drop((store, o));
    assert!(
        !dir.path().join("backups").exists(),
        "ohne Migration keine Sicherung"
    );
    allowed_files_only(&dir);
}

#[test]
fn stand_2_wird_erst_gesichert_dann_migriert() {
    let base = tmp("t5-stand2");
    let dir = data_dir_in(&base).unwrap();
    let vault = MemoryStore::default();
    {
        let conn = db::open(&dir.db_path(), &vault).unwrap();
        conn.execute_batch(daedyx_tasks_core::schema::STAND_1)
            .unwrap();
        conn.execute_batch(daedyx_tasks_core::schema::STAND_2)
            .unwrap();
        conn.execute(
            "INSERT INTO task_meta (key, value) VALUES ('schema_version', '2')",
            [],
        )
        .unwrap();
    }
    let o = open(&dir, &vault);
    assert_eq!(version(&o), SCHEMA_VERSION.to_string());
    drop(o);
    assert!(
        dir.path()
            .join("backups")
            .join("daedyx-tasks-v2.db")
            .exists()
    );
    allowed_files_only(&dir);
}

#[test]
fn ohne_datenordner_der_echten_app_und_ohne_dateien_daneben() {
    let base = tmp("t5-daneben");
    let dir = data_dir_in(&base).unwrap();
    let _ = open(&dir, &MemoryStore::default());
    let names: Vec<String> = std::fs::read_dir(&base)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        names,
        [crate::ids::APP_ID.to_string()],
        "nur der eigene Ordner entsteht"
    );
    let _ = checksum(&dir.db_path());
}
