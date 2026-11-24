//! Review T0, M-1: Unsinnige Werte in der Datenbank (veränderte Datei, später ein Abgleich) dürfen den Takt nie zum
//! Absturz bringen. Ungültige Erinnerungen werden übersprungen, gültige weiter gemeldet, die Datenbank bleibt gleich.
use super::*;
use daedyx_tasks_core::{edit_alarms, edit_lists, edit_tasks};
use rusqlite::types::Value as SqlValue;

// 2026-11-19 09:00 Berlin = 08:00 UTC (Aufgaben ohne Uhrzeit erinnern um 9 Uhr Ortszeit).
const NINE: i64 = 1_795_075_200;

fn fresh() -> (Connection, String) {
    let conn = Connection::open_in_memory().unwrap();
    daedyx_tasks_core::migrate(&conn).unwrap();
    let list = edit_lists::create_list(&conn, "Test", "", 1000).unwrap().id;
    (conn, list)
}

fn task(conn: &Connection, list: &str, title: &str, alarms: &[i64]) -> String {
    let f = serde_json::json!({"title": title, "due": "2026-11-19"});
    let id = edit_tasks::create_task(conn, list, None, f.as_object().unwrap(), 1000).unwrap().id;
    edit_alarms::set_alarms(conn, &id, alarms, 1000).unwrap();
    id
}

/// Alle Zeilen der Tabellen, die der Takt liest oder schreibt.
fn dump(conn: &Connection) -> Vec<String> {
    let mut out = Vec::new();
    for table in ["tasks", "task_alarms", "task_alarms_fired"] {
        let mut st = conn.prepare(&format!("SELECT * FROM {table} ORDER BY 1, 2")).unwrap();
        let n = st.column_count();
        let rows =
            st.query_map([], |r| (0..n).map(|i| r.get::<_, SqlValue>(i)).collect::<Result<Vec<_>, _>>()).unwrap();
        out.extend(rows.map(|r| format!("{table}: {:?}", r.unwrap())));
    }
    out
}

/// Ein Durchgang; eine Panik macht den Test rot statt ihn abzubrechen.
fn tick_without_panic(db: &Mutex<Connection>, now: i64) -> Vec<String> {
    let mut seen = Vec::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tick_once(db, now, "Europe/Berlin", |r| seen.push(r.title.clone()));
    }));
    assert!(result.is_ok(), "Panik im Erinnerungstakt");
    seen
}

#[test]
fn absurd_reminder_minutes_in_the_database_are_skipped_without_panic() {
    for bad in [i64::MAX, i64::MIN, -1, 525_601] {
        let (conn, list) = fresh();
        let id = task(&conn, &list, "Kaputte Erinnerung", &[5]);
        conn.execute("UPDATE task_alarms SET minutes_before = ?1 WHERE task_id = ?2", (bad, &id)).unwrap();
        let before = dump(&conn);
        let db = Mutex::new(conn);
        assert!(tick_without_panic(&db, NINE).is_empty(), "{bad}");
        assert_eq!(dump(&db.lock().unwrap()), before, "Datenbank verändert ({bad})");
    }
}

#[test]
fn an_absurd_due_time_in_the_database_is_skipped_without_panic() {
    for bad in [i64::MIN, i64::MAX, -1, 253_402_300_800] {
        let (conn, list) = fresh();
        let id = task(&conn, &list, "Kaputte Fälligkeit", &[1]);
        conn.execute("UPDATE tasks SET due = ?1, due_has_time = 1 WHERE id = ?2", (bad.to_string(), &id)).unwrap();
        let before = dump(&conn);
        let db = Mutex::new(conn);
        for now in [NINE, 0, 253_402_300_799] {
            assert!(tick_without_panic(&db, now).is_empty(), "{bad} / {now}");
        }
        assert_eq!(dump(&db.lock().unwrap()), before, "Datenbank verändert ({bad})");
    }
}

#[test]
fn a_broken_task_does_not_hide_the_valid_ones() {
    let (conn, list) = fresh();
    task(&conn, &list, "Gültige Aufgabe", &[0]);
    let bad = task(&conn, &list, "Kaputte Erinnerung", &[5]);
    conn.execute("UPDATE task_alarms SET minutes_before = ?1 WHERE task_id = ?2", (i64::MAX, &bad)).unwrap();
    task(&conn, &list, "Noch eine gültige", &[0, 10]);
    let db = Mutex::new(conn);
    let mut seen = tick_without_panic(&db, NINE);
    seen.sort();
    assert_eq!(seen, vec!["Gültige Aufgabe", "Noch eine gültige", "Noch eine gültige"]);
}

#[test]
fn the_checked_scan_matches_the_core_for_valid_data() {
    let (conn, list) = fresh();
    task(&conn, &list, "Ganztägig", &[30]);
    let timed = task(&conn, &list, "Mit Uhrzeit", &[60]);
    conn.execute("UPDATE tasks SET due = '1795096800', due_has_time = 1 WHERE id = ?1", [&timed]).unwrap();
    task(&conn, &list, "Mehrere", &[0, 10, 1440, 525_600]);
    let f = serde_json::json!({"title": "Altes Einzelfeld", "due": "2026-11-20", "remind_minutes": 15});
    edit_tasks::create_task(&conn, &list, None, f.as_object().unwrap(), 1000).unwrap();
    let windows = [(0, MAX_SECS), (NINE - 3600, NINE + 1), (NINE, NINE + 86_400), (NINE - 2 * 86_400, NINE)];
    let compare = |conn: &Connection| {
        for (from, to) in windows {
            for tz in ["Europe/Berlin", "UTC", "America/New_York"] {
                let ours = pending_checked(conn, from, to, tz).unwrap();
                let core = daedyx_tasks_core::reminders::pending(conn, from, to, tz).unwrap();
                assert_eq!(ours, core, "{from}..{to} {tz}");
            }
        }
    };
    assert!(pending_checked(&conn, 0, MAX_SECS, "Europe/Berlin").unwrap().len() >= 6);
    compare(&conn);
    assert!(tick_checked(&conn, NINE, "Europe/Berlin", |_| {}).unwrap() > 0);
    compare(&conn);
}

/// Review T0, N-7: Falsche Datentypen (SQLite erlaubt sie in jeder Spalte) schalten nur die betroffene Zeile ab.
#[test]
fn a_row_with_a_wrong_data_type_does_not_silence_the_other_reminders() {
    let (conn, list) = fresh();
    task(&conn, &list, "Gültige Aufgabe", &[0]);
    let blob = task(&conn, &list, "Titel wird Binärwert", &[0]);
    conn.execute("UPDATE tasks SET title = X'00FF' WHERE id = ?1", [&blob]).unwrap();
    let mixed = task(&conn, &list, "Eine Erinnerung kaputt", &[0, 5]);
    conn.execute("UPDATE task_alarms SET minutes_before = 'abc' WHERE task_id = ?1 AND minutes_before = 5", [&mixed])
        .unwrap();
    let all_text = task(&conn, &list, "Alle Erinnerungen kaputt", &[0]);
    conn.execute("UPDATE task_alarms SET minutes_before = 'abc' WHERE task_id = ?1", [&all_text]).unwrap();
    let data = |c: &Connection| dump(c).into_iter().filter(|l| !l.starts_with("task_alarms_fired")).collect::<Vec<_>>();
    let before = data(&conn);
    let db = Mutex::new(conn);
    let mut seen = tick_without_panic(&db, NINE);
    seen.sort();
    // „Alle Erinnerungen kaputt“ fällt nicht auf das alte Einzelfeld zurück: die Aufgabe hat eigene (kaputte) Zeilen.
    assert_eq!(seen, vec!["Eine Erinnerung kaputt", "Gültige Aufgabe"]);
    assert_eq!(data(&db.lock().unwrap()), before, "Aufgaben oder Erinnerungen verändert");
}
