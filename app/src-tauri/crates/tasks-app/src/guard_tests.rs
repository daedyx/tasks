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
