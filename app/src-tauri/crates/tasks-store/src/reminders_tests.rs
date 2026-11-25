use super::*;
use daedyx_tasks_core::{edit_lists, edit_tasks};
use serde_json::{json, Map, Value};

// 2026-11-19 09:00 Berlin = 08:00 UTC (Aufgaben ohne Uhrzeit erinnern um 9 Uhr Ortszeit).
const NINE: i64 = 1_795_075_200;

fn db_with_due_task() -> Mutex<Connection> {
    let conn = Connection::open_in_memory().unwrap();
    daedyx_tasks_core::migrate(&conn).unwrap();
    let list = edit_lists::create_list(&conn, "Test", "", 1000).unwrap();
    let f: Map<String, Value> =
        json!({"title": "Erfundene Aufgabe", "due": "2026-11-19", "remind_minutes": 0}).as_object().unwrap().clone();
    edit_tasks::create_task(&conn, &list.id, None, &f, 1000).unwrap();
    Mutex::new(conn)
}

#[test]
fn a_due_reminder_is_reported_exactly_once() {
    let db = db_with_due_task();
    let mut seen = Vec::new();
    assert_eq!(tick_once(&db, NINE, "Europe/Berlin", |r| seen.push(r.title.clone())), 1);
    assert_eq!(seen, vec!["Erfundene Aufgabe".to_string()]);
    assert_eq!(tick_once(&db, NINE + 30, "Europe/Berlin", |_| seen.push("doppelt".into())), 0);
    assert_eq!(seen.len(), 1);
}

#[test]
fn nothing_is_reported_before_the_time() {
    let db = db_with_due_task();
    assert_eq!(tick_once(&db, NINE - 60, "Europe/Berlin", |_| {}), 0);
}

#[test]
fn a_poisoned_lock_skips_the_tick_without_panic() {
    let db = db_with_due_task();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = db.lock().unwrap();
        panic!("Panik an anderer Stelle, während die Sperre gehalten wird");
    }));
    assert!(db.is_poisoned());
    assert_eq!(tick_once(&db, NINE, "Europe/Berlin", |_| {}), 0);
}

#[test]
fn hostile_clock_and_zone_values_do_not_panic() {
    let db = db_with_due_task();
    for now in [0, -1, i64::MIN, i64::MAX] {
        let _ = tick_once(&db, now, "Europe/Berlin", |_| {});
    }
    for tz in ["", "Mars/Olympus", "../../etc", &"x".repeat(10_000)] {
        let _ = tick_once(&db, NINE, tz, |_| {});
    }
}

#[test]
fn the_clock_is_after_2020() {
    assert!(unix_now() > 1_577_836_800);
    assert!(!local_zone().is_empty());
}

fn fired_count(db: &Mutex<Connection>) -> i64 {
    db.lock().unwrap().query_row("SELECT COUNT(*) FROM task_alarms_fired", [], |r| r.get(0)).unwrap()
}

#[test]
fn a_clock_jumping_far_ahead_and_back_does_not_repeat_a_reminder() {
    // Review T4, N4-2: Die Uhr springt drei Tage vor und wieder zurück; der Merker der noch gültigen Erinnerung bleibt.
    let db = db_with_due_task();
    assert_eq!(tick_once(&db, NINE, "Europe/Berlin", |_| {}), 1);
    assert_eq!(tick_once(&db, NINE + 3 * 86_400, "Europe/Berlin", |_| {}), 0);
    assert_eq!(fired_count(&db), 1, "Merker einer gültigen Erinnerung bleibt");
    assert_eq!(tick_once(&db, NINE + 60, "Europe/Berlin", |_| {}), 0, "nicht ein zweites Mal");
}

#[test]
fn markers_of_finished_or_changed_reminders_are_pruned_after_two_days() {
    let db = db_with_due_task();
    assert_eq!(tick_once(&db, NINE, "Europe/Berlin", |_| {}), 1);
    assert_eq!(tick_once(&db, NINE + 86_400, "Europe/Berlin", |_| {}), 0);
    assert_eq!(fired_count(&db), 1, "jünger als zwei Tage bleibt immer");
    db.lock().unwrap().execute("UPDATE tasks SET done_at = 1", []).unwrap();
    assert_eq!(tick_once(&db, NINE + 3 * 86_400, "Europe/Berlin", |_| {}), 0);
    assert_eq!(fired_count(&db), 0, "erledigt und älter als zwei Tage: weg");
}
