//! Daedyx Tasks in reinem Rust (egui), Meilenstein T2 „Gerüst“: Rahmen nach Stil 1 mit Navigationsleiste (Liste,
//! Board, Einstellungen), Kopf- und Statusleiste, die Aufgaben-Oberfläche aus `daedyx-tasks-ui` auf einer
//! Speicher-Datenbank mit erfundenen Aufgaben, Startfehler-Fenster in egui. Nur Kennungen der Vorschau.
pub mod app;
pub mod ids;
pub mod launch;
pub mod page;
mod prefs;
pub mod seed;
pub mod settings;
pub mod shell;
pub mod startfehler;
mod status;
pub mod texts;

#[cfg(test)]
mod app_tests;
#[cfg(test)]
mod guard_tests;
#[cfg(test)]
mod startfehler_tests;
#[cfg(test)]
mod test_support;
