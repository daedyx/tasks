//! Lizenzangaben der Vorschau (Tasks T6, wie Desk Review int2 M-1): Jede mitgebaute Bibliothek hat Urheber und
//! Lizenztext, der mitgebaute C-Code (SQLCipher, OpenSSL) steht darin, und die Liste stimmt in beide Richtungen mit
//! `Cargo.lock` überein. „Über“ zeigt die Liste, der Installer liefert dieselben Texte als `Lizenzen.txt`.
use crate::prefs::license_texts::{Gap, TEXTS, full_text, missing};
use crate::prefs::licenses::{License, licenses, matches};
use eframe::egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};

const LOCK: &str = include_str!("../../../Cargo.lock");
const EXCLUDED: &str = include_str!("../lizenzen-ausgenommen.tsv");
const NSI: &str = include_str!("../../../../../installer/tasks-vorschau.nsi");
const HINT: &str = "Liste neu erzeugen: node scripts/vorschau-lizenzen.mjs";

/// (Name, Version) aller Pakete aus Cargo.lock mit Quelle (ohne eigene Pfad-Pakete).
fn locked() -> Vec<(String, String)> {
    let lock = LOCK.replace("\r\n", "\n");
    let mut out = Vec::new();
    for block in lock.split("[[package]]\n").skip(1) {
        let field = |key: &str| {
            block
                .lines()
                .find_map(|l| l.strip_prefix(key))
                .map(|v| v.trim_matches('"').to_owned())
        };
        if let (Some(name), Some(version), Some(_)) =
            (field("name = "), field("version = "), field("source = "))
        {
            out.push((name, version));
        }
    }
    out
}

/// Eintrag für mitgebauten C-Code (steht nicht als eigenes Paket in Cargo.lock).
fn is_c_code(l: &License) -> bool {
    l.name.contains("(C-Code")
}

#[test]
fn jede_bibliothek_hat_urheber_und_lizenztext() {
    let gaps = missing(&licenses(), TEXTS);
    assert!(gaps.is_empty(), "ohne Lizenztext ({HINT}): {gaps:?}");
    assert!(licenses().len() > 100, "Liste unvollständig");
}

#[test]
fn fehlende_lizenztexte_werden_erkannt() {
    let texts = "== a 1.0 ==\nlizenz: MIT\nurheber: X\ntexte: [T1]\n\n== b 2.0 ==\nlizenz: MIT\nurheber: Y\ntexte: [T9]\n\n== c 3.0 ==\nlizenz: MIT\n\n=== Lizenztexte ===\n\n--- [T1] ---\nPermission is hereby granted\n";
    let list =
        [("a", "1.0"), ("b", "2.0"), ("c", "3.0"), ("d", "4.0")].map(|(name, version)| License {
            name,
            version,
            license: "MIT",
        });
    let gaps = missing(&list, texts);
    let gap = |n: &str| gaps.iter().find(|g| g.0 == n).map(|g| g.2.clone());
    assert_eq!(gaps.len(), 3, "{gaps:?}");
    assert_eq!(gap("b"), Some(Gap::Reference("[T9]".into())));
    assert_eq!(gap("c"), Some(Gap::Text));
    assert_eq!(gap("d"), Some(Gap::Section));
    assert!(
        full_text("a", "1.0", texts).is_some_and(|t| t.contains("Permission is hereby granted"))
    );
}

#[test]
fn c_code_und_pflichttexte_sind_dabei() {
    let list = licenses();
    assert!(
        list.iter().any(|l| l.name.starts_with("SQLCipher (C-Code")),
        "SQLCipher fehlt"
    );
    assert!(
        list.iter().any(|l| l.name.starts_with("OpenSSL (C-Code")),
        "OpenSSL fehlt"
    );
    for needle in [
        "Zetetic",
        "Apache License",
        "SIL OPEN FONT LICENSE",
        "UBUNTU FONT LICENCE",
    ] {
        assert!(TEXTS.contains(needle), "„{needle}“ fehlt");
    }
    assert!(TEXTS.starts_with("Lizenzhinweise der Bibliotheken in Daedyx Tasks (Vorschau)"));
}

/// Richtung 1: Jedes Paket aus Cargo.lock steht in der Liste oder in `lizenzen-ausgenommen.tsv` (nicht mitgebaut).
#[test]
fn jedes_paket_aus_cargo_lock_ist_erfasst() {
    let list = licenses();
    let gaps: Vec<String> = locked()
        .into_iter()
        .filter(|(n, v)| {
            let listed = list.iter().any(|l| l.name == n && l.version == v);
            let excluded = EXCLUDED
                .lines()
                .any(|l| l.trim_end() == format!("{n}\t{v}"));
            !listed && !excluded
        })
        .map(|(n, v)| format!("{n} {v}"))
        .collect();
    assert!(
        gaps.is_empty(),
        "nicht erfasst ({HINT}):\n{}",
        gaps.join("\n")
    );
}

/// Richtung 2: Nichts in Liste oder Ausnahmen, was es in Cargo.lock nicht (mehr) gibt; nichts steht in beiden.
#[test]
fn liste_und_ausnahmen_enthalten_nur_pakete_aus_cargo_lock() {
    let lock = locked();
    let in_lock = |n: &str, v: &str| lock.iter().any(|(ln, lv)| ln == n && lv == v);
    let list = licenses();
    let stale: Vec<String> = list
        .iter()
        .filter(|l| !is_c_code(l) && !in_lock(l.name, l.version))
        .map(|l| format!("{} {}", l.name, l.version))
        .chain(EXCLUDED.lines().filter_map(|line| {
            let (n, v) = line.trim_end().split_once('\t')?;
            (!in_lock(n, v)).then(|| format!("{n} {v} (ausgenommen)"))
        }))
        .collect();
    assert!(
        stale.is_empty(),
        "nicht in Cargo.lock ({HINT}):\n{}",
        stale.join("\n")
    );
    let both: Vec<&License> = list
        .iter()
        .filter(|l| {
            EXCLUDED
                .lines()
                .any(|x| x.trim_end() == format!("{}\t{}", l.name, l.version))
        })
        .collect();
    assert!(
        both.is_empty(),
        "zugleich gelistet und ausgenommen: {both:?}"
    );
}

#[test]
fn filter_nach_name_oder_lizenz() {
    let l = License {
        name: "webpki-roots",
        version: "1.0.0",
        license: "CDLA-Permissive-2.0",
    };
    assert!(matches(&l, ""));
    assert!(matches(&l, "  WEBPKI "));
    assert!(matches(&l, "cdla"));
    assert!(!matches(&l, "gpl"));
}

/// „Über“: Liste mit Anzahl, Filter, und ein gewählter Eintrag zeigt Urheber und vollständigen Lizenztext.
#[test]
fn ueber_zeigt_liste_und_lizenztext_eines_eintrags() {
    let mut h = crate::t4_ui_tests::settings_page();
    let count = licenses().len().to_string();
    let _ = h.get_by_label(&format!("Verwendete Bibliotheken ({count})"));
    let _ = h.get_by_label("Eintrag wählen, um Urheber und Lizenztext zu sehen.");
    h.get_by_role_and_label(Role::TextInput, "Bibliotheken filtern")
        .focus();
    h.run();
    h.get_by_role_and_label(Role::TextInput, "Bibliotheken filtern")
        .type_text("rusqlite");
    h.run();
    h.run();
    let entry = licenses()
        .into_iter()
        .find(|l| l.name == "rusqlite")
        .unwrap();
    let label = format!("rusqlite {}", entry.version);
    assert!(
        h.query_by_label(&format!(
            "eframe {}",
            licenses()
                .into_iter()
                .find(|l| l.name == "eframe")
                .unwrap()
                .version
        ))
        .is_none(),
        "Filter wirkt"
    );
    h.get_by_label(&label).click();
    h.run();
    h.run();
    assert!(
        h.query_all_by_label_contains("Permission is hereby granted")
            .next()
            .is_some(),
        "Lizenztext fehlt"
    );
    assert!(
        h.query_all_by_role(Role::Heading)
            .any(|n| n.accesskit_node().label().as_deref() == Some(label.as_str()))
    );
}

/// Der Installer liefert die Lizenzhinweise mit und entfernt sie bei der Deinstallation (Vorlage aus daedyx-build).
#[test]
fn installer_liefert_die_lizenztexte_mit() {
    assert!(
        NSI.contains(r"crates\tasks-app\lizenztexte.txt"),
        "Quelle fehlt"
    );
    assert!(
        daedyx_build::installer::NSI.contains(r#"File "/oname=Lizenzen.txt" "${LICENSES}""#),
        "nicht installiert"
    );
    assert!(
        daedyx_build::installer::NSI.contains(r#"Delete "$INSTDIR\Lizenzen.txt""#),
        "nicht deinstalliert"
    );
}
