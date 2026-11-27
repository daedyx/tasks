//! Produkt-Setup (T7, Lücken L2 und L3, Review U3 F-U3-5): `installer/tasks.nsi` trägt die echten Kennungen aus
//! `ids.rs` (Merkmal `produkt`), das Programm heißt `daedyx-tasks.exe`, und der Uninstall-Schlüssel ist
//! `Daedyx Tasks`, genau der Name, unter dem der Launcher Tasks in der Registry sucht (`launcher-core` `apps.rs`,
//! `registry_key`). Die Vorlage aus daedyx-build bildet Ordner-, Uninstall- und Verknüpfungsnamen aus `PRODUCT`.
use crate::ids::{APP_ID, PREVIEW, RUN_VALUE, UPDATE_APP};

const NSI: &str = include_str!("../../../../../installer/tasks.nsi");
const VORSCHAU_NSI: &str = include_str!("../../../../../installer/tasks-vorschau.nsi");
const CARGO: &str = include_str!("../Cargo.toml");
const WORKFLOW: &str = include_str!("../../../../../.github/workflows/release.yml");
/// Produktname, zugleich Uninstall-Schlüssel (`HKCU\...\Uninstall\Daedyx Tasks`) und Ordner-Merkschlüssel.
const PRODUCT: &str = "Daedyx Tasks";
/// Den Namen erwartet der Launcher (F-U3-5); ändert er sich, findet der Launcher die Installation nicht mehr.
const LAUNCHER_REGISTRY_KEY: &str = "Daedyx Tasks";
const EXE: &str = "daedyx-tasks.exe";

#[test]
fn produkt_setup_traegt_die_echten_kennungen() {
    // Git legt die Datei unter Windows mit CRLF ab.
    let nsi = NSI.replace("\r\n", "\n");
    assert!(nsi.contains(&format!("!define PRODUCT \"{PRODUCT}\"\n")));
    assert!(nsi.contains(&format!("!define EXE \"{EXE}\"\n")));
    assert!(nsi.contains("!define AUMID \"de.daedyx.tasks\"\n"));
    assert!(nsi.contains(r#"!include "${VORLAGE}\app-vorschau.nsi""#));
    // Keine Kennung der Vorschau im Produkt-Setup (außer im erklärenden Kopf).
    let code: String = nsi
        .lines()
        .filter(|l| !l.trim_start().starts_with(';'))
        .collect();
    for bad in ["vorschau.exe", "tasks.vorschau", "Tasks Vorschau"] {
        assert!(!code.contains(bad), "{bad}");
    }
    // Die Vorschau bleibt getrennt: anderer Produktname, also andere Schlüssel.
    assert!(VORSCHAU_NSI.contains("!define PRODUCT \"Daedyx Tasks Vorschau\""));
    if !PREVIEW {
        assert_eq!(APP_ID, "de.daedyx.tasks");
        assert_eq!(RUN_VALUE, "DaedyxTasks");
        assert_eq!(UPDATE_APP, "daedyx-tasks");
    }
}

#[test]
fn registry_name_ist_der_den_der_launcher_sucht() {
    assert_eq!(PRODUCT, LAUNCHER_REGISTRY_KEY);
    let v = daedyx_build::installer::NSI;
    assert!(v.contains(r#"!define REGKEY "${PRODUCT}""#));
    assert!(v.contains(
        r#"!define UNINSTKEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${REGKEY}""#
    ));
    assert!(v.contains(r#"WriteRegStr HKCU "${UNINSTKEY}" "DisplayName" "${PRODUCT}""#));
    // Der Launcher liest `app.json` im Installationsordner: Programmname aus `EXE`.
    assert!(v.contains(r#"FileWrite $0 '{"version": "${VERSION}", "exe": "${EXE}"}'"#));
    assert!(v.contains(r#"InstallDir "$LOCALAPPDATA\${PRODUCT}""#));
}

#[test]
fn programm_der_produkt_fassung_heisst_daedyx_tasks() {
    // Eigenes Programmziel nur mit `produkt` (L3); die Vorschau behält ihren Namen.
    let bin = "[[bin]]\nname = \"daedyx-tasks\"\npath = \"src/main_produkt.rs\"\nrequired-features = [\"produkt\"]\n";
    assert!(CARGO.replace("\r\n", "\n").contains(bin), "{CARGO}");
    assert!(CARGO.contains("name = \"daedyx-tasks-vorschau\""));
    // Der Release-Lauf baut genau dieses Programm in genau dieses Setup.
    let yml = WORKFLOW.replace("\r\n", "\n");
    assert!(yml.contains("$nsi = 'installer/tasks.nsi'"));
    assert!(yml.contains(&format!("-Exe {EXE} ")));
    assert!(
        yml.contains("--features produkt --bin daedyx-tasks\n"),
        "{yml}"
    );
}
