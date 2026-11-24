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
