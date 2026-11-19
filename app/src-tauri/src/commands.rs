//! Fehlerform aller Tauri-Befehle: `{code, message}`; die Oberfläche übersetzt den Code,
//! die Meldung enthält nie Zugangsdaten.
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CommandError {
    pub code: &'static str,
    pub message: String,
}
