//! Datenquelle der Vorschau: Speicher-Datenbank mit erfundenen Aufgaben (`testdata/aufgaben.json`, wie Desk M6a),
//! befüllt über `daedyx_tasks_ui::seed` mit den normalen Kern-Funktionen. Keine Datei, kein Tresor (Entscheid T-F2).
//! Oberfläche und Takt der Erinnerungen teilen die Datenbank (T4).
use crate::reminders::{SharedConn, SharedDb};
use chrono_tz::Tz;
use daedyx_tasks_ui::{Clock, TaskStore};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

const DEMO: &str = include_str!("../testdata/aufgaben.json");

/// Zeitzone des Systems; unbekannt: Europe/Berlin.
pub fn zone() -> Tz {
    iana_time_zone::get_timezone()
        .ok()
        .and_then(|name| name.parse::<Tz>().ok())
        .unwrap_or(chrono_tz::Europe::Berlin)
}

/// Leere Speicher-Datenbank mit dem Kern-Schema, geteilt für Oberfläche und Takt.
pub fn shared() -> Option<SharedConn> {
    let conn = Connection::open_in_memory().ok()?;
    conn.pragma_update(None, "foreign_keys", "ON").ok()?;
    daedyx_tasks_core::migrate(&conn).ok()?;
    Some(Arc::new(Mutex::new(conn)))
}

/// Vorschau mit erfundenen Aufgaben und der geteilten Datenbank. Scheitert das Befüllen, bleibt sie leer; scheitert
/// schon die Datenbank, gibt es keine (die Oberfläche zeigt dann leere Ansichten, der Takt meldet nichts).
pub fn demo_shared(tz: Tz, clock: Clock) -> Option<(TaskStore, SharedConn)> {
    let db = shared()?;
    let store = TaskStore::on(Box::new(SharedDb(Arc::clone(&db))), tz, clock);
    let _ = daedyx_tasks_ui::seed::fill(&store, DEMO);
    Some((store, db))
}

/// Vorschau mit erfundenen Aufgaben, ohne Takt (Tests der Oberfläche).
pub fn demo(tz: Tz, clock: Clock) -> Option<TaskStore> {
    demo_shared(tz, clock).map(|(store, _)| store)
}
