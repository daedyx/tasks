//! Aufteilung des Release-Laufs (Review T7v, N-T7v-4 und M-T7v-3): Jeder Job hat nur die Rechte für seine Aufgabe, und
//! Programme aus dem Bau laufen nie in einem Job mit Rechten zum Schreiben oder mit der Kennung für den Nachweis.
//! - `bauen`: testet und baut, nur `contents: read`.
//! - `bezeugen`: Freigabe (Environment `release`), `id-token` und `attestations: write`; führt nichts aus dem Bau aus.
//! - `pruefen`: nur `contents: read`; hier läuft das Prüfprogramm aus dem Bau (M-T7v-2), danach kein Schritt mit Rechten.
//! - `veroeffentlichen`: nur `contents: write`; legt das Release an, führt nichts aus dem Bau aus.
use super::workflow;

const NAMES: [&str; 4] = ["bauen", "bezeugen", "pruefen", "veroeffentlichen"];

/// Der Workflow-Text der vier Jobs in [`NAMES`], in dieser Reihenfolge.
fn jobs(yml: &str) -> [&str; 4] {
    let at = NAMES.map(|n| {
        yml.find(&format!("\n  {n}:\n"))
            .unwrap_or_else(|| panic!("{n}"))
    });
    assert!(
        at.iter().zip(at.iter().skip(1)).all(|(a, b)| a < b),
        "{at:?}"
    );
    let mut ends = at.iter().skip(1).copied().chain([yml.len()]);
    at.map(|start| yml.get(start..ends.next().unwrap()).unwrap())
}

/// Genau diese Rechte, und keine weiteren Zeilen im Block `permissions:`.
fn permissions(job: &str) -> Vec<&str> {
    job.lines()
        .skip_while(|l| *l != "    permissions:")
        .skip(1)
        .take_while(|l| l.starts_with("      "))
        .map(|l| l.split('#').next().unwrap().trim())
        .collect()
}

#[test]
fn jeder_job_hat_nur_seine_rechte() {
    let yml = workflow();
    let [build, attest, check, publish] = jobs(&yml);
    assert_eq!(permissions(build), ["contents: read"]);
    assert_eq!(
        permissions(attest),
        ["id-token: write", "attestations: write"]
    );
    assert_eq!(permissions(check), ["contents: read"]);
    assert_eq!(permissions(publish), ["contents: write"]);
    // Die Freigabe sitzt dort, wo die Kennung entsteht (Review T7v, Nachtrag 2).
    assert!(attest.contains("    environment: release\n"));
    for job in [build, check, publish] {
        assert!(!job.contains("environment:"), "{job}");
    }
    assert!(attest.contains("    needs: bauen\n"));
    assert!(check.contains("    needs: [bauen, bezeugen]\n"));
    assert!(publish.contains("    needs: [bauen, bezeugen, pruefen]\n"));
}

#[test]
fn programme_aus_dem_bau_laufen_nur_ohne_rechte() {
    let yml = workflow();
    let [build, attest, check, publish] = jobs(&yml);
    for bad in ["attest-build-provenance", "gh release create"] {
        assert!(!build.contains(bad), "bauen: {bad}");
    }
    // Kein Bau, kein Quelltext, keine Werkzeugkette nach dem Bau-Job.
    for job in [attest, check, publish] {
        for bad in [
            "cargo ",
            "rustup",
            "actions/checkout",
            "make-entry",
            "installer-bauen",
        ] {
            assert!(!job.contains(bad), "{bad}: {job}");
        }
    }
    // Das Prüfprogramm aus dem Bau läuft nur im Job ohne Rechte, und nichts aus der Übergabe wird anderswo gestartet.
    let tool = "release_selbstpruefung.exe";
    assert!(check.contains(&format!("& ./uebergabe/{tool} ")));
    for job in [attest, publish] {
        assert!(!job.contains(tool), "{job}");
        assert!(!job.contains("& ./") && !job.contains("& .\\"), "{job}");
    }
    // Nachweis nur in `bezeugen`, Release nur in `veroeffentlichen`.
    assert!(attest.contains("uses: actions/attest-build-provenance@"));
    for job in [check, publish] {
        assert!(!job.contains("attest-build-provenance"), "{job}");
    }
    assert!(publish.contains("gh release create"));
    for job in [attest, check] {
        assert!(!job.contains("gh release create"), "{job}");
    }
}

#[test]
fn uebergaben_sind_mit_pruefsummen_gegengeprueft() {
    let yml = workflow();
    let [build, attest, check, publish] = jobs(&yml);
    assert!(build.contains("uses: actions/upload-artifact@"));
    assert!(attest.contains("uses: actions/upload-artifact@"));
    for job in [attest, check, publish] {
        assert!(job.contains("uses: actions/download-artifact@"), "{job}");
        assert!(job.contains("needs.bauen.outputs.pruefsummen"), "{job}");
    }
    // Das Bündel aus `bezeugen` gilt nur mit dessen Prüfsumme (Job-Ausgabe, nicht aus dem Artefakt).
    assert!(attest.contains("nachweis: ${{ steps.nachweis_summe.outputs.nachweis }}"));
    for job in [check, publish] {
        assert!(job.contains("needs.bezeugen.outputs.nachweis"), "{job}");
    }
    // In `pruefen` setzt kein Schritt eine Ausgabe, auf die ein späterer Job baut.
    assert!(!check.contains("outputs:"), "{check}");
    assert!(!publish.contains("needs.pruefen.outputs"), "{publish}");
}

#[test]
fn release_workflow_prueft_mit_dem_pruefweg_der_app_vor_dem_veroeffentlichen() {
    let yml = workflow();
    let [build, _, check, _] = jobs(&yml);
    // Das Prüfprogramm entsteht im Bau-Job mit der Produkt-Fassung …
    let tool = "cargo build --release --locked --offline -p daedyx-tasks-app --features produkt \
                --example release_selbstpruefung";
    assert!(build.contains(tool), "{build}");
    // … und läuft vor `gh attestation verify`; beides muss grün sein, bevor `veroeffentlichen` startet (`needs`).
    let at = |needle: &str| check.find(needle).unwrap_or_else(|| panic!("{needle}"));
    assert!(at("release_selbstpruefung.exe") < at("gh attestation verify"));
    // Die Version muss neuer sein als das bisherige Latest (H-T7v-1), geprüft vor dem Bau.
    let b = |needle: &str| build.find(needle).unwrap_or_else(|| panic!("{needle}"));
    assert!(b("gh release view --repo") < b("cargo build"));
    // entry.json schreibt die getestete Windows PowerShell 5.1 (N-T7v-3).
    let step = build
        .get(b("- name: entry.json schreiben")..b("make-entry.ps1 -App "))
        .unwrap();
    assert!(step.contains("shell: powershell\n"), "{step}");
}
