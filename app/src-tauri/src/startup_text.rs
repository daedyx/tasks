//! Texte des Startfehler-Dialogs. Sie stehen in der Sprachdatei der Oberfläche (Namensraum `start.`),
//! weil beim Fehler noch keine Oberfläche läuft.
use std::collections::HashMap;
use std::sync::OnceLock;

static DE: OnceLock<HashMap<String, String>> = OnceLock::new();

fn table() -> &'static HashMap<String, String> {
    DE.get_or_init(|| serde_json::from_str(include_str!("../../src/i18n/de.json")).unwrap_or_default())
}

/// Text zum Schlüssel mit ersetzten `{platzhaltern}`; bei unbekanntem Schlüssel der Schlüssel selbst.
pub fn text(key: &str, vars: &[(&str, &str)]) -> String {
    let mut out = table().get(key).cloned().unwrap_or_else(|| key.to_string());
    for (name, value) in vars {
        out = out.replace(&format!("{{{name}}}"), value);
    }
    out
}
