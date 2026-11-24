//! Erinnerungstakt als reine Funktion: Ein Durchgang bekommt Datenbank, Uhrzeit und Zeitzone von außen und meldet
//! Fälliges über einen Rückruf. Wann der Takt läuft (Thread, Tokio, Tauri) und wie gemeldet wird, entscheidet die Hülle.
use daedyx_tasks_core::reminders::{tick, DueReminder};
use rusqlite::Connection;
use std::sync::Mutex;
use std::time::Duration;

pub use daedyx_tasks_core::reminders::DueReminder as Reminder;

/// Wartezeit nach dem Start bis zum ersten Durchgang (die Oberfläche soll zuerst stehen).
pub const FIRST_TICK_DELAY: Duration = Duration::from_secs(4);
/// Abstand zwischen zwei Durchgängen.
pub const TICK_INTERVAL: Duration = Duration::from_secs(30);

/// Jetzt in Unix-Sekunden; vor 1970 oder nicht darstellbar ergibt 0 (dann ist nichts fällig, nie eine Panik).
pub fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|d| i64::try_from(d.as_secs()).ok())
        .unwrap_or(0)
}

/// Zeitzone des Rechners (IANA-Name), sonst `UTC`.
pub fn local_zone() -> String {
    iana_time_zone::get_timezone().unwrap_or_else(|_| "UTC".to_string())
}

/// Letzte Sekunde, die der Takt annimmt (9999-12-31 23:59:59 UTC). Der Kern rechnet mit `now` ungeprüft
/// (`now - 2 Tage`, `now + 1`); außerhalb von `0..=MAX_NOW` liefe er über und bräche das Programm ab.
const MAX_NOW: i64 = 253_402_300_799;

/// Ein Durchgang: meldet jede fällige Erinnerung genau einmal über `fire` und merkt sie sich in der Datenbank.
/// Gibt die Zahl der Meldungen zurück. Eine unsinnige Uhrzeit, eine verdorbene Sperre (Panik an anderer Stelle) oder
/// ein Datenbankfehler lassen den Durchgang still aus; der nächste versucht es wieder. Die Sperre wird nur für diesen
/// Durchgang gehalten.
pub fn tick_once(db: &Mutex<Connection>, now: i64, tz: &str, fire: impl FnMut(&DueReminder)) -> usize {
    if !(0..=MAX_NOW).contains(&now) {
        return 0;
    }
    match db.lock() {
        Ok(conn) => tick(&conn, now, tz, fire).unwrap_or(0),
        Err(_) => 0,
    }
}

#[cfg(test)]
#[path = "reminders_tests.rs"]
mod tests;
