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
        // Kein loser Endungstext: Er könnte im Programm direkt hinter einer echten Kennung liegen und dann Byte für
        // Byte wie die Vorschau-Kennung aussehen (Review T2, N1-1). Nur die zwei vollständigen Namen in `ids.rs`.
        let endings = src.matches("vorschau\"").count();
        if f.ends_with("ids.rs") {
            assert_eq!(endings, 2, "nur APP_ID und VAULT der Vorschau");
        } else {
            assert!(!src.contains("de.daedyx.tasks"), "{}", f.display());
            assert!(
                !src.contains("DaedyxTasks"),
                "Autostart-Wert: {}",
                f.display()
            );
            assert!(
                !src.contains("tasks-vorschau"),
                "Tresor/Pipe: {}",
                f.display()
            );
            assert_eq!(endings, 0, "{}", f.display());
        }
    }
}

/// Plan 4.2: Kennung und Tresor kommen beide aus `ids.rs` und gehören zur selben Fassung; der Store bekommt den Dienst
/// übergeben. `cfg!` statt `#[cfg(...)]`, damit es keine Ausnahme-Stelle wird.
#[test]
fn kennung_und_tresor_gehoeren_zur_selben_fassung() {
    use crate::ids::{APP_ID, BACKGROUND, PIPE_PREFIX, RUN_VALUE, VAULT, consistent, vault};
    let (id, service, run, pipe) = if cfg!(feature = "produkt") {
        (
            "de.daedyx.tasks",
            "daedyx-tasks",
            "DaedyxTasks",
            "daedyx-tasks-instanz",
        )
    } else {
        (
            "de.daedyx.tasks.vorschau",
            "daedyx-tasks-vorschau",
            "DaedyxTasksVorschau",
            "daedyx-tasks-vorschau-instanz",
        )
    };
    assert_eq!((APP_ID, VAULT), (id, service));
    assert_eq!((RUN_VALUE, PIPE_PREFIX), (run, pipe));
    assert_eq!(
        (
            BACKGROUND.aumid,
            BACKGROUND.run_value,
            BACKGROUND.pipe_prefix
        ),
        (id, run, pipe)
    );
    assert_eq!(vault().service(), VAULT);
    assert!(consistent());
}

/// Gemischte Fassungen fallen auf (Startprobe endet dann mit 1). Namen hier nur als Testdaten, nicht im Programm.
#[test]
fn gemischte_fassungen_sind_nicht_dieselbe() {
    use crate::ids::same_version;
    let (pid, pvault) = ("de.daedyx.tasks.vorschau", "daedyx-tasks-vorschau");
    let (rid, rvault) = ("de.daedyx.tasks", "daedyx-tasks");
    assert!(same_version(pid, pvault, true));
    assert!(same_version(rid, rvault, false));
    assert!(!same_version(pid, rvault, true));
    assert!(!same_version(rid, pvault, true));
    assert!(!same_version(pid, pvault, false));
    assert!(!same_version(rid, rvault, true));
    use crate::ids::same_background;
    let (prun, ppipe) = ("DaedyxTasksVorschau", "daedyx-tasks-vorschau-instanz");
    let (rrun, rpipe) = ("DaedyxTasks", "daedyx-tasks-instanz");
    assert!(same_background(pvault, prun, ppipe));
    assert!(same_background(rvault, rrun, rpipe));
    assert!(
        !same_background(pvault, rrun, ppipe),
        "echter Autostart-Wert"
    );
    assert!(!same_background(pvault, prun, rpipe), "echte Pipe");
    assert!(!same_background(rvault, prun, rpipe));
    assert!(
        !same_background(rvault, rrun, ppipe),
        "Vorschau-Pipe im Produkt"
    );
    assert!(!same_background(pvault, prun, "daedyx-tasks-vorschau-"));
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
