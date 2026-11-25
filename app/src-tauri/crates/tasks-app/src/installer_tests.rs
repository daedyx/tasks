//! Setup der Vorschau (Tasks T6): Installer und Probe tragen dieselben Kennungen wie das Programm (`ids.rs`), nur die
//! der Vorschau, und binden die gemeinsame Vorlage aus daedyx-build ein. Ein echter Lauf ist die Installationsprobe.
use crate::ids::{APP_ID, RUN_VALUE};

const NSI: &str = include_str!("../../../../../installer/tasks-vorschau.nsi");
const BAUEN: &str = include_str!("../../../../../scripts/installer-vorschau.ps1");
const PROBE: &str = include_str!("../../../../../scripts/installer-probe-vorschau.ps1");
const EXE: &str = "daedyx-tasks-vorschau.exe";
const PRODUCT: &str = "Daedyx Tasks Vorschau";

#[test]
fn installer_traegt_die_kennungen_der_vorschau() {
    if cfg!(feature = "produkt") {
        return; // Produkt-Setup kommt erst mit T7.
    }
    assert!(NSI.contains(&format!("!define PRODUCT \"{PRODUCT}\"")));
    assert!(NSI.contains(&format!("!define AUMID \"{APP_ID}\"")));
    assert!(NSI.contains(&format!("!define EXE \"{EXE}\"")));
    assert!(NSI.contains(r#"!include "${VORLAGE}\app-vorschau.nsi""#));
    assert!(
        BAUEN.contains(&format!("-Exe \"{EXE}\""))
            && BAUEN.contains("-Package \"daedyx-tasks-app\"")
    );
    assert!(BAUEN.contains(r"installer\tasks-vorschau.nsi"));
    for want in [
        format!("-Product \"{PRODUCT}\""),
        format!("-Exe \"{EXE}\""),
        format!("-Aumid \"{APP_ID}\""),
        format!("-RunValue \"{RUN_VALUE}\""),
    ] {
        assert!(PROBE.contains(&want), "Probe: {want}");
    }
}

/// Der Ordner-Merkschlüssel der Vorlage folgt aus dem Produktnamen: `HKCU\Software\daedyx\Daedyx Tasks Vorschau`.
#[test]
fn merkschluessel_der_vorschau_aus_der_vorlage() {
    let v = daedyx_build::installer::NSI;
    assert!(v.contains(r#"!define DIRKEY "Software\daedyx\${PRODUCT}""#));
    assert!(
        v.contains("RequestExecutionLevel user"),
        "nur pro Benutzer, keine Adminrechte"
    );
    assert!(PRODUCT.ends_with(" Vorschau") && APP_ID.ends_with(".vorschau"));
}

/// Die Probe bricht vor jedem Schreiben ab, wenn die echte App installiert ist, und prüft sie danach unverändert.
#[test]
fn probe_schuetzt_die_echte_app() {
    let abort = PROBE.find("if (Test-RealInstalled)").unwrap();
    let first_write = PROBE.find("installer-probe.ps1\") -Setup").unwrap();
    assert!(abort < first_write, "Abbruch vor dem ersten Schreiben");
    assert!(PROBE.contains(r#"Uninstall\Daedyx Tasks""#));
    assert!(PROBE.contains(r#"Software\daedyx\Daedyx Tasks""#));
    assert!(PROBE.contains("(Get-RealState) -eq $before"));
    // Datenordner der echten App (Review T6, N6-1): vorhanden = Abbruch vor dem ersten Schreiben, am Ende unverändert.
    let real_data = PROBE
        .find(r#"Join-Path $env:APPDATA "de.daedyx.tasks""#)
        .unwrap();
    assert!(
        real_data < first_write,
        "Schutz der echten Daten vor dem ersten Schreiben"
    );
    assert!(PROBE.contains("(Get-RealDataState) -eq $realDataBefore"));
    // Datenordner der Vorschau: vorher vorhanden = Abbruch (keine fremden Daten löschen), nachher muss er fehlen
    // (`--cleanup` entfernt ihn ab T5).
    let data = PROBE.find("if (Test-Path -LiteralPath $dataDir)").unwrap();
    assert!(
        data < first_write,
        "Datenordner-Schutz vor dem ersten Schreiben"
    );
    assert!(PROBE.contains(r#"Join-Path $env:APPDATA "de.daedyx.tasks.vorschau""#));
    assert!(PROBE.contains("if (-not (Test-Path -LiteralPath $dataDir))"));
    assert!(
        !PROBE.contains("Remove-Item") && !PROBE.contains("Set-ItemProperty"),
        "nur lesen"
    );
}
