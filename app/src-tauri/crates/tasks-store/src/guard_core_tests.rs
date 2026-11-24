//! Wächter (Review T0, M-1 Rest): Der ungeprüfte Erinnerungs-Durchgang des Kerns (`reminders::tick`, `::pending`)
//! rechnet mit Werten aus der Datenbank ohne Überlaufprüfung. Weder Hülle noch Store-Crate dürfen ihn aufrufen oder
//! weiterreichen; der Takt läuft nur über `daedyx_tasks_store::reminders::tick_once`. Ausnahme: der Gleichheitstest
//! `reminders_hostile_tests.rs`, der beide Fassungen vergleicht.
use std::path::{Path, PathBuf};

const EXEMPT: &str = "reminders_hostile_tests.rs";

/// Text ohne Leerraum und ohne `//`-Kommentare, damit Umbrüche in `use`-Listen nichts verstecken.
fn squeeze(text: &str) -> String {
    text.lines().map(|l| l.split("//").next().unwrap_or("")).collect::<String>().split_whitespace().collect()
}

/// Inhalt der ersten `{…}`-Liste ab `start` (ohne Verschachtelung, reicht für `use`-Listen).
fn braces(s: &str, start: usize) -> &str {
    let rest = s.get(start..).unwrap_or("");
    rest.find('}').and_then(|end| rest.get(..end)).unwrap_or(rest)
}

fn has_word(list: &str, word: &str) -> bool {
    list.split([',', '{', '}', ':']).any(|w| w == word || w.starts_with(&format!("{word}as")))
}

/// Gibt den ersten verbotenen Zugang zum ungeprüften Kern-Durchgang zurück.
fn unchecked_core_tick(text: &str) -> Option<&'static str> {
    let s = squeeze(text);
    let module = "daedyx_tasks_core::reminders";
    for bad in ["::tick", "::pending", ";", "as", "::*"] {
        if s.contains(&format!("{module}{bad}")) {
            return Some("Kern-reminders direkt");
        }
    }
    for (i, _) in s.match_indices(&format!("{module}::{{")) {
        let list = braces(&s, i);
        if has_word(list, "tick") || has_word(list, "pending") || has_word(list, "self") {
            return Some("tick/pending aus dem Kern importiert");
        }
    }
    for (i, _) in s.match_indices("daedyx_tasks_core::{") {
        if has_word(braces(&s, i), "reminders") {
            return Some("Kern-Modul reminders eingebunden oder weitergereicht");
        }
    }
    s.contains("daedyx_tasks_core::*").then_some("ganzer Kern eingebunden")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn nobody_reaches_the_unchecked_core_tick() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    rust_files(&root.join("../../src"), &mut files);
    assert!(files.len() > 20, "zu wenige Dateien gefunden: {}", files.len());
    let hits: Vec<_> = files
        .iter()
        .filter(|p| !p.ends_with(EXEMPT) && !p.ends_with("guard_core_tests.rs"))
        .filter_map(|p| unchecked_core_tick(&std::fs::read_to_string(p).unwrap()).map(|why| format!("{p:?}: {why}")))
        .collect();
    assert!(hits.is_empty(), "{hits:#?}");
}

#[test]
fn the_guard_sees_every_way_in() {
    for bad in [
        "use daedyx_tasks_core::reminders::tick;",
        "daedyx_tasks_core::reminders::pending(&c, 0, 1, \"UTC\")",
        "use daedyx_tasks_core::reminders;",
        "use daedyx_tasks_core::reminders as r;",
        "use daedyx_tasks_core::reminders::{\n    DueReminder,\n    tick as t,\n};",
        "use daedyx_tasks_core::reminders::{self, DueReminder};",
        "pub use daedyx_tasks_core::{\n    edit_alarms, reminders, repeat,\n};",
        "pub use daedyx_tasks_core::{reminders::tick, store};",
        "use daedyx_tasks_core::*;",
    ] {
        assert!(unchecked_core_tick(bad).is_some(), "nicht erkannt: {bad}");
    }
    for ok in [
        "use daedyx_tasks_core::reminders::{mark_fired, DueReminder, CATCH_UP_SECS};",
        "pub use daedyx_tasks_core::reminders::DueReminder as Reminder;",
        "pub use daedyx_tasks_core::{edit_alarms, repeat, store};",
        "// daedyx_tasks_core::reminders::tick nur im Kommentar",
    ] {
        assert!(unchecked_core_tick(ok).is_none(), "fälschlich gemeldet: {ok}");
    }
}
