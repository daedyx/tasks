//! Texte der App: eigene Schlüssel aus `lang/{de,en,fr}.json`, dazu die der Aufgaben-Oberfläche (`aufgaben.*`) aus
//! `daedyx-tasks-ui` und die des Update-Bauteils (`update.*`) aus `daedyx-updater`, zur Laufzeit zusammengeführt
//! (keine Abschrift, anders als in Desk).
use daedyx_ui::i18n::{Language, Texts};
use serde_json::{Map, Value};

/// Eigene Sprachdatei der App.
pub fn source(language: Language) -> &'static str {
    match language {
        Language::De => include_str!("../lang/de.json"),
        Language::En => include_str!("../lang/en.json"),
        Language::Fr => include_str!("../lang/fr.json"),
    }
}

fn map(json: &str) -> Map<String, Value> {
    serde_json::from_str(json).unwrap_or_default()
}

/// Alle Texte einer Sprache. Eine kaputte Datei (verhindert ein Test) zeigt `[[schlüssel]]` statt abzustürzen.
pub fn texts(language: Language) -> Texts {
    let mut all = map(daedyx_tasks_ui::texts::source(language));
    all.extend(map(daedyx_updater::texts::source(language)));
    all.extend(map(source(language)));
    let json = Value::Object(all).to_string();
    Texts::from_json(&json).unwrap_or_default()
}
