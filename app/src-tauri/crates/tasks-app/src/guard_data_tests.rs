//! Wächter um Daten und Tresor (Plan 4.2, T5, Review T5 M5-1 und N5-1): Tests benutzen nie den echten Tresor und nie
//! den echten Datenordner, und das Programm prüft die Einzelinstanz, bevor es die Datenbank öffnet.
use std::path::{Path, PathBuf};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().is_some_and(|e| e == "rs") {
            out.push(p);
        }
    }
}

/// Testdateien von App und Store, ohne die Testdatei des Tresor-Moduls und ohne diese Datei.
fn test_files(root: &Path, store: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for dir in [root.join("src"), root.join("tests"), store.to_path_buf()] {
        walk(&dir, &mut files);
    }
    files.retain(|f| {
        let name = f.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        (name.ends_with("_tests.rs") || name == "tests.rs" || f.starts_with(root.join("tests")))
            && !f.ends_with(Path::new("secrets").join("tests.rs"))
            && !f.ends_with(Path::new("src").join("guard_data_tests.rs"))
    });
    files
}

/// Wege zum echten Tresor oder zum echten Datenordner aus einem Test: `KeyringStore`, `ids::vault()` (außer zum
/// Lesen des Dienstnamens), `paths::data_dir()`/`data_base()` (Tests nehmen `data_dir_in` mit Wegwerf-Ordner),
/// `open_data` und `launch::run`. Ein Programmtest startet das Programm als eigenen Prozess, das sieht dieser Wächter
/// nicht; der setzt `APPDATA` selbst auf einen Wegwerf-Ordner.
fn forbidden(src: &str) -> Vec<&'static str> {
    let mut found = Vec::new();
    for word in [
        "KeyringStore",
        "data_dir(",
        "data_base(",
        "open_data",
        "launch::run",
    ] {
        if src.contains(word) {
            found.push(word);
        }
    }
    if src.matches("vault()").count() != src.matches("vault().service()").count() {
        found.push("vault()");
    }
    found
}

#[test]
fn tests_benutzen_nie_den_echten_tresor_oder_datenordner() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let store = root.join("../tasks-store/src");
    let files = test_files(root, &store);
    assert!(files.len() > 15, "Testdateien gefunden: {}", files.len());
    for f in files {
        let src = std::fs::read_to_string(&f).unwrap();
        assert!(
            forbidden(&src).is_empty(),
            "{}: {:?}",
            f.display(),
            forbidden(&src)
        );
    }
    let vault_tests = std::fs::read_to_string(store.join("secrets").join("tests.rs")).unwrap();
    let handtest = vault_tests.find("fn keyring_store_roundtrip").unwrap();
    let before = vault_tests.get(..handtest).unwrap();
    let attrs = before.get(before.rfind("#[test]").unwrap()..).unwrap();
    assert!(
        attrs.contains("#[ignore"),
        "der Handtest am echten Tresor bleibt ignoriert"
    );
}

/// Der Wächter sieht auch die Umwege (Gegenprobe ohne Datei).
#[test]
fn der_waechter_sieht_die_umwege() {
    for bad in [
        "data::open(&d, &ids::vault(), &f, z, c)",
        "let d = paths::data_dir();",
        "paths::data_base()",
        "launch::run(s, a)",
        "open_data(zone)",
        "KeyringStore::new()",
    ] {
        assert!(!forbidden(bad).is_empty(), "{bad}");
    }
    assert!(forbidden("assert_eq!(vault().service(), VAULT); data_dir_in(&base)").is_empty());
}

/// Review T5, M5-1: In `launch::run` steht die Einzelinstanz (`instance::acquire`) vor dem Öffnen der Datenbank.
#[test]
fn einzelinstanz_vor_der_datenbank() {
    let src = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("launch.rs"),
    )
    .unwrap();
    let run = src.find("pub fn run(").unwrap();
    let body = src.get(run..).unwrap();
    let acquire = body.find("instance::acquire").unwrap();
    let open = body.find("open_data(zone)").unwrap();
    assert!(
        acquire < open,
        "Datenbank erst nach der Einzelinstanz öffnen"
    );
    assert_eq!(body.matches("open_data(").count(), 1);
}
