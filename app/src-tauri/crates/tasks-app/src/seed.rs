//! Datenquelle der Vorschau: Speicher-Datenbank mit erfundenen Aufgaben (`testdata/aufgaben.json`, wie Desk M6a),
//! befüllt über `daedyx_tasks_ui::seed` mit den normalen Kern-Funktionen. Keine Datei, kein Tresor (Entscheid T-F2).
use chrono_tz::Tz;
use daedyx_tasks_ui::{Clock, TaskStore};

const DEMO: &str = include_str!("../testdata/aufgaben.json");

/// Zeitzone des Systems; unbekannt: Europe/Berlin.
pub fn zone() -> Tz {
    iana_time_zone::get_timezone()
        .ok()
        .and_then(|name| name.parse::<Tz>().ok())
        .unwrap_or(chrono_tz::Europe::Berlin)
}

/// Vorschau mit erfundenen Aufgaben. Scheitert das Befüllen, bleibt sie leer; scheitert schon die Datenbank, gibt es
/// keine (die Oberfläche zeigt dann leere Ansichten).
pub fn demo(tz: Tz, clock: Clock) -> Option<TaskStore> {
    let store = TaskStore::memory(tz, clock).ok()?;
    let _ = daedyx_tasks_ui::seed::fill(&store, DEMO);
    Some(store)
}
