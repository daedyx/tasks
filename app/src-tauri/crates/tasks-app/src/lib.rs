//! Daedyx Tasks in reinem Rust (egui): Rahmen nach Stil 1 mit Navigationsleiste (Liste, Board, Einstellungen), die
//! Aufgaben-Oberfläche aus `daedyx-tasks-ui`, Hintergrund (T4) und ab T5 eine eigene verschlüsselte Datenbank im
//! Datenordner der Vorschau mit erfundenen Aufgaben, Einstellungen in `task_meta`, Startfehler-Fenster in egui. Nur
//! Kennungen der Vorschau (ohne Merkmal `produkt`).
pub mod app;
mod background;
pub mod cleanup;
pub mod data;
pub mod ids;
pub mod launch;
pub mod page;
pub mod paths;
mod prefs;
#[cfg(not(feature = "produkt"))]
pub mod probe;
#[cfg(feature = "produkt")]
#[path = "probe_off.rs"]
pub mod probe;
pub mod reminders;
pub mod seed;
pub mod settings;
pub mod shell;
pub mod startfehler;
mod status;
pub mod stored;
pub mod texts;
pub mod update;

#[cfg(test)]
mod app_tests;
#[cfg(test)]
mod cleanup_tests;
#[cfg(test)]
mod data_error_tests;
#[cfg(test)]
mod data_tests;
#[cfg(test)]
mod guard_data_tests;
#[cfg(test)]
mod guard_tests;
#[cfg(test)]
mod installer_tests;
#[cfg(test)]
mod license_tests;
#[cfg(test)]
mod release_tests;
#[cfg(test)]
mod reminder_tests;
#[cfg(test)]
mod startfehler_tests;
#[cfg(test)]
mod stored_tests;
#[cfg(test)]
mod t4_ui_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod update_net_tests;
#[cfg(test)]
mod update_tests;
#[cfg(test)]
mod update_ui_tests;
#[cfg(all(test, windows))]
mod windows_tests;
