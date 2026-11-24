//! Datenhaltung von Daedyx Tasks ohne Oberfläche (Meilenstein T0): verschlüsselte Datenbank (SQLCipher), Schlüssel im
//! Tresor, Sicherung vor jeder Migration, Startfehler als Codes (Texte macht die Oberfläche) und der Erinnerungstakt
//! als reine Funktion. Kein Tauri, kein Netz. Schema und Rechenregeln gehören dem Aufgaben-Kern `daedyx-tasks-core`.
pub mod db;
pub mod limits;
pub mod reminders;
pub mod secrets;
pub mod startup;
#[cfg(any(test, feature = "test-support"))]
pub mod test_support;

#[cfg(test)]
mod guard_core_tests;
#[cfg(test)]
mod guard_tests;
#[cfg(test)]
mod startup_tests;
