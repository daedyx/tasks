//! Review T4, H4-6: Der Takt räumt alte Merker höchstens einmal je Stunde auf, meldet aber in jedem Durchgang. Nur
//! Speicher-Datenbank, erfundene Aufgabe.
use super::*;
use daedyx_tasks_core::{edit_lists, edit_tasks};
use serde_json::{json, Map, Value};

// 2026-11-19 09:00 Berlin = 08:00 UTC.
const NINE: i64 = 1_795_075_200;
const THREE_DAYS: i64 = 3 * 24 * 3600;

fn db() -> Mutex<Connection> {
    let conn = Connection::open_in_memory().unwrap();
    daedyx_tasks_core::migrate(&conn).unwrap();
    let list = edit_lists::create_list(&conn, "Test", "", 1000).unwrap();
    let f: Map<String, Value> =
        json!({"title": "Erfundene Aufgabe", "due": "2026-11-19", "remind_minutes": 0}).as_object().unwrap().clone();
    edit_tasks::create_task(&conn, &list.id, None, &f, 1000).unwrap();
    Mutex::new(conn)
}

/// Merker einer Aufgabe, die es nicht gibt (älter als zwei Tage): den räumt jede Aufräumrunde weg.
fn stale(db: &Mutex<Connection>) {
    db.lock()
        .unwrap()
        .execute("INSERT INTO task_alarms_fired (task_id, fire_at) VALUES ('weg', ?1)", [NINE - THREE_DAYS])
        .unwrap();
}

fn stale_left(db: &Mutex<Connection>) -> i64 {
    db.lock()
        .unwrap()
        .query_row("SELECT count(*) FROM task_alarms_fired WHERE task_id = 'weg'", [], |r| r.get(0))
        .unwrap()
}

#[test]
fn aufgeraeumt_wird_beim_ersten_durchgang_und_dann_hoechstens_einmal_je_stunde() {
    let db = db();
    let mut takt = Takt::default();
    stale(&db);
    let mut seen = 0;
    assert_eq!(takt.tick(&db, NINE, "Europe/Berlin", |_| seen += 1), 1, "meldet sofort");
    assert_eq!(stale_left(&db), 0, "erster Durchgang räumt auf");
    stale(&db);
    takt.tick(&db, NINE + 30, "Europe/Berlin", |_| seen += 1);
    takt.tick(&db, NINE + 1800, "Europe/Berlin", |_| seen += 1);
    assert_eq!(stale_left(&db), 1, "innerhalb der Stunde kein Aufräumen");
    takt.tick(&db, NINE + PRUNE_EVERY, "Europe/Berlin", |_| seen += 1);
    assert_eq!(stale_left(&db), 0, "nach einer Stunde wieder");
    assert_eq!(seen, 1, "jede Erinnerung genau einmal");
}

#[test]
fn springt_die_uhr_zurueck_wird_wieder_aufgeraeumt() {
    let db = db();
    let mut takt = Takt::default();
    takt.tick(&db, NINE, "Europe/Berlin", |_| {});
    stale(&db);
    takt.tick(&db, NINE - 60, "Europe/Berlin", |_| {});
    assert_eq!(stale_left(&db), 0);
}

#[test]
fn uhr_ausserhalb_1970_bis_9999_laesst_den_durchgang_aus() {
    let db = db();
    let mut takt = Takt::default();
    assert_eq!(takt.tick(&db, -1, "Europe/Berlin", |_| {}), 0);
    assert_eq!(takt.tick(&db, MAX_SECS + 1, "Europe/Berlin", |_| {}), 0);
}
