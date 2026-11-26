//! Einstellungen der App in der Tabelle `task_meta` des Kerns (Plan 4.4, Tasks T5): Schlüssel/Wert, keine
//! Schemaänderung. Nur Schlüssel der Positivliste werden gelesen oder geschrieben; die Werte sind kurzer Text. Was die
//! App daraus macht (unbekannt oder kaputt = Standardwert), entscheidet sie selbst. Die Einträge sind nur auf diesem
//! Gerät, der Abgleich des Kerns trägt `task_meta` nicht.
use rusqlite::{Connection, OptionalExtension};

pub const THEME: &str = "app.theme";
pub const LANGUAGE: &str = "app.language";
pub const KEEP_RUNNING: &str = "app.keep_running";
pub const NOTIFY_TITLES: &str = "app.notify_titles";
pub const DARK_TUNING: &str = "app.dark_tuning";

/// Feste Schlüssel (Plan 4.4). Den Update-Zustand (Schalter, letzte Suche) führt seit U2 `updates.json` im
/// Datenordner (`daedyx-update`, gemeinsam mit dem Launcher), nicht `task_meta`.
pub const KEYS: [&str; 5] = [THEME, LANGUAGE, KEEP_RUNNING, NOTIFY_TITLES, DARK_TUNING];
/// Board-Modus je Liste: `app.board_mode.<Listen-Kennung>`.
pub const BOARD_MODE: &str = "app.board_mode.";
/// Längster erlaubter Wert in Zeichen; mehr ist kein gültiger Wert dieser Einstellungen.
pub const MAX_VALUE: usize = 200;
/// Längste Listen-Kennung im Schlüssel (der Kern vergibt 32 Hex-Zeichen).
const MAX_LIST_ID: usize = 64;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PrefsError {
    #[error("Einstellung nicht erlaubt")]
    Key,
    #[error("Wert zu lang oder mit Steuerzeichen")]
    Value,
    #[error("Datenbank: {0}")]
    Db(String),
}

/// Steht der Schlüssel auf der Positivliste?
pub fn allowed(key: &str) -> bool {
    if KEYS.contains(&key) {
        return true;
    }
    key.strip_prefix(BOARD_MODE).is_some_and(|id| {
        !id.is_empty() && id.len() <= MAX_LIST_ID && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
    })
}

fn db(e: rusqlite::Error) -> PrefsError {
    PrefsError::Db(e.to_string())
}

/// Wert eines erlaubten Schlüssels; unbekannte Schlüssel, fehlende oder unbrauchbare Werte ergeben `None`.
pub fn get(conn: &Connection, key: &str) -> Option<String> {
    if !allowed(key) {
        return None;
    }
    let value: Option<String> =
        conn.query_row("SELECT value FROM task_meta WHERE key = ?1", [key], |r| r.get(0)).optional().ok().flatten();
    value.filter(|v| valid_value(v))
}

fn valid_value(v: &str) -> bool {
    v.chars().count() <= MAX_VALUE && !v.chars().any(char::is_control)
}

/// Schreibt einen erlaubten Schlüssel (ersetzt den alten Wert).
pub fn set(conn: &Connection, key: &str, value: &str) -> Result<(), PrefsError> {
    if !allowed(key) {
        return Err(PrefsError::Key);
    }
    if !valid_value(value) {
        return Err(PrefsError::Value);
    }
    conn.execute(
        "INSERT INTO task_meta (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        (key, value),
    )
    .map(|_| ())
    .map_err(db)
}

/// Entfernt einen erlaubten Schlüssel (zurück zum Standardwert der App).
pub fn remove(conn: &Connection, key: &str) -> Result<(), PrefsError> {
    if !allowed(key) {
        return Err(PrefsError::Key);
    }
    conn.execute("DELETE FROM task_meta WHERE key = ?1", [key]).map(|_| ()).map_err(db)
}

/// Alle gespeicherten Board-Modi (Listen-Kennung, Wert), nur gültige Einträge.
pub fn board_modes(conn: &Connection) -> Vec<(String, String)> {
    let Ok(mut q) = conn.prepare("SELECT key, value FROM task_meta WHERE key LIKE 'app.board_mode.%'") else {
        return Vec::new();
    };
    let rows = q.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)));
    rows.map(|it| {
        it.filter_map(Result::ok)
            .filter(|(k, v)| allowed(k) && valid_value(v))
            .filter_map(|(k, v)| k.strip_prefix(BOARD_MODE).map(|id| (id.to_owned(), v)))
            .collect()
    })
    .unwrap_or_default()
}

#[cfg(test)]
#[path = "prefs_tests.rs"]
mod tests;
