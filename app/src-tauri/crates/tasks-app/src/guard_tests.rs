//! Wächter über das Crate: jeder benutzte Schlüssel steht in den Texten, keine festen Texte, Farben nur aus den
//! Tokens; die drei Sprachen haben dieselben Schlüssel; die echte Kennung steht nur in `ids.rs` (hinter dem Merkmal
//! `produkt`). `ids.rs` enthält nur technische Kennungen und wird wie in Desk nur auf Farben geprüft.
use crate::texts::{source, texts};
use daedyx_ui::i18n::{Language, Texts};
use daedyx_ui::lint::{check_files, color_values, rust_files};
use std::path::{Path, PathBuf};

fn files() -> Vec<PathBuf> {
    rust_files(&[Path::new(env!("CARGO_MANIFEST_DIR")).join("src")])
}

#[test]
fn ui_regeln_gelten_fuer_die_app() {
    let (ids, rest): (Vec<_>, Vec<_>) = files().into_iter().partition(|f| f.ends_with("ids.rs"));
    assert_eq!(ids.len(), 1);
    assert!(rest.len() >= 10, "Quelltexte gefunden: {}", rest.len());
    let src = std::fs::read_to_string(&ids[0]).unwrap();
    assert!(color_values("ids.rs", &src).is_empty());
    for lang in Language::ALL {
        let found = check_files(&rest, &texts(lang), &[]);
        assert!(
            found.is_empty(),
            "{lang:?}: Verstöße:\n{}",
            found.join("\n")
        );
    }
}

fn keys(lang: Language) -> Vec<String> {
    Texts::from_json(source(lang))
        .unwrap()
        .entries()
        .map(|(k, _)| k.clone())
        .collect()
}

#[test]
fn alle_sprachen_haben_dieselben_app_schluessel() {
    let de = keys(Language::De);
    assert!(de.len() >= 20);
    assert_eq!(de, keys(Language::En));
    assert_eq!(de, keys(Language::Fr));
    assert!(
        de.iter().all(|k| !k.starts_with("aufgaben.")),
        "aufgaben.* kommt aus daedyx-tasks-ui"
    );
}

#[test]
fn echte_kennung_nur_in_ids_rs() {
    for f in files() {
        let src = std::fs::read_to_string(&f).unwrap();
        if !f.ends_with("ids.rs") {
            assert!(!src.contains("de.daedyx.tasks"), "{}", f.display());
        }
    }
}

/// Plan 4.2: Kennung und Tresor kommen beide aus `ids.rs` und gehören zur selben Fassung; der Store bekommt den Dienst
/// übergeben. `cfg!` statt `#[cfg(...)]`, damit es keine Ausnahme-Stelle wird.
#[test]
fn kennung_und_tresor_gehoeren_zur_selben_fassung() {
    use crate::ids::{APP_ID, VAULT, consistent, vault};
    let (id, service) = if cfg!(feature = "produkt") {
        ("de.daedyx.tasks", "daedyx-tasks")
    } else {
        ("de.daedyx.tasks.vorschau", "daedyx-tasks-vorschau")
    };
    assert_eq!((APP_ID, VAULT), (id, service));
    assert_eq!(vault().service(), VAULT);
    assert!(consistent());
}

/// `Connection::open` ist im ganzen Workspace per Clippy gesperrt (Plan 4.2); der Store öffnet nur an einer Stelle mit
/// eingetragener Ausnahme (A-030). Eine zweite `clippy.toml` ist nach der Regel des Prüfwerkzeugs nicht erlaubt.
#[test]
fn clippy_sperrt_das_oeffnen_der_datenbank_ausserhalb_des_stores() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let clippy = std::fs::read_to_string(dir.join("../../clippy.toml")).unwrap();
    for method in [
        "rusqlite::Connection::open\"",
        "rusqlite::Connection::open_with_flags\"",
    ] {
        assert!(clippy.contains(method), "{method}");
    }
    let store = dir.join("../tasks-store/src/db/mod.rs");
    let src = std::fs::read_to_string(store).unwrap();
    assert_eq!(
        src.matches("Connection::open(").count(),
        1,
        "eine Öffnungsstelle"
    );
    assert!(src.contains("A-030"));
}
