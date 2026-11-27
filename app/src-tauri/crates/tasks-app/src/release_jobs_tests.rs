//! Aufteilung des Release-Laufs (Review T7v, N-T7v-4, Weg 1): Der Job `bauen` testet und baut ohne Schreibrechte und
//! ohne Kennung für den Nachweis; nur `veroeffentlichen` hat Freigabe, `id-token` und `contents: write` und führt keinen
//! Bau und keinen fremden Quelltext aus. Dazu die Reihenfolge: bezeugen, mit dem Prüfweg der App prüfen (M-T7v-2),
//! erst dann veröffentlichen.
use super::workflow;

/// Der Workflow-Text der beiden Jobs (`bauen`, `veroeffentlichen`).
fn jobs(yml: &str) -> (&str, &str) {
    let build = yml.find("\n  bauen:\n").unwrap();
    let publish = yml.find("\n  veroeffentlichen:\n").unwrap();
    assert!(build < publish);
    (
        yml.get(build..publish).unwrap(),
        yml.get(publish..).unwrap(),
    )
}

#[test]
fn bauen_hat_keine_schreibrechte_und_veroeffentlichen_baut_nichts() {
    let yml = workflow();
    let (build, publish) = jobs(&yml);
    assert!(
        build.contains("    permissions:\n      contents: read\n"),
        "{build}"
    );
    for bad in [
        "write",
        "id-token",
        "environment:",
        "attest-build-provenance",
        "gh release create",
    ] {
        assert!(!build.contains(bad), "bauen: {bad}");
    }
    assert!(publish.contains("    needs: bauen\n"));
    assert!(publish.contains("    environment: release\n"));
    for right in ["contents: write", "id-token: write", "attestations: write"] {
        assert!(publish.contains(right), "veroeffentlichen: {right}");
    }
    // Kein Bau, kein Quelltext, keine Werkzeugkette im Job mit der Kennung für den Nachweis.
    for bad in [
        "cargo ",
        "rustup",
        "actions/checkout",
        "make-entry",
        "installer-bauen",
    ] {
        assert!(!publish.contains(bad), "veroeffentlichen: {bad}");
    }
    // Übergabe nur als Artefakt, mit Prüfsummen aus dem Bau-Job gegengeprüft.
    assert!(build.contains("uses: actions/upload-artifact@"));
    assert!(publish.contains("uses: actions/download-artifact@"));
    assert!(publish.contains("needs.bauen.outputs.pruefsummen"));
}

#[test]
fn release_workflow_prueft_mit_dem_pruefweg_der_app_vor_dem_veroeffentlichen() {
    let yml = workflow();
    let (build, publish) = jobs(&yml);
    // Das Prüfprogramm entsteht im Bau-Job mit der Produkt-Fassung …
    let tool = "cargo build --release --locked --offline -p daedyx-tasks-app --features produkt \
                --example release_selbstpruefung";
    assert!(build.contains(tool), "{build}");
    // … und läuft im Veröffentlichen-Job nach dem Nachweis und vor dem Release.
    let at = |needle: &str| publish.find(needle).unwrap_or_else(|| panic!("{needle}"));
    let check = "release_selbstpruefung.exe";
    assert!(at("actions/attest-build-provenance@") < at(check));
    assert!(at(check) < at("gh attestation verify"));
    assert!(at("gh attestation verify") < at("gh release create"));
    // Die Version muss neuer sein als das bisherige Latest (H-T7v-1), geprüft vor dem Bau.
    let b = |needle: &str| build.find(needle).unwrap_or_else(|| panic!("{needle}"));
    assert!(b("gh release view --repo") < b("cargo build"));
    // entry.json schreibt die getestete Windows PowerShell 5.1 (N-T7v-3).
    let step = build
        .get(b("- name: entry.json schreiben")..b("make-entry.ps1 -App "))
        .unwrap();
    assert!(step.contains("shell: powershell\n"), "{step}");
}
