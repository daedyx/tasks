//! T7-Vorbereitung (F-U2-2): Der Release-Workflow passt zum Prüfweg des Kerns. Er liegt genau unter
//! `policy::WORKFLOW`, läuft nur auf `policy::GIT_REF` im eigenen Repo, braucht kein Secret und baut die
//! Produkt-Fassung. Ein erfundenes Release, wie der Workflow es baut (`scripts/ci/make-entry.ps1`, Dateinamen und
//! Subjekte aus `release.yml`), besteht Suche und Download mit der festen Quelle dieser App. Die Sigstore-Kette selbst
//! prüft der Kern gegen ein echtes Bündel derselben Form (`release.yml@refs/heads/main`, astral-sh/uv); hier steht eine
//! Attrappe, die genau die Subjekte aus `release.yml` kennt.
use crate::update::{OWNER, REPO};
use daedyx_updater::daedyx_update::check::{BUNDLE_FILE, ENTRY_FILE};
use daedyx_updater::daedyx_update::policy::{GIT_REF, WORKFLOW};
use std::path::{Path, PathBuf};

const VERSION: &str = "9.9.9";
/// So setzt der Workflow die Version in Dateinamen ein.
const VERSION_EXPR: &str = "${{ steps.eingaben.outputs.version }}";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

/// Der Workflow mit LF-Zeilenenden (Git legt ihn unter Windows mit CRLF ab).
fn workflow() -> String {
    std::fs::read_to_string(repo_root().join(WORKFLOW))
        .unwrap()
        .replace("\r\n", "\n")
}

/// Dateinamen unter `subject-path: |` (eine Zeile je Datei), Version eingesetzt.
fn subjects(yml: &str) -> Vec<String> {
    yml.lines()
        .skip_while(|l| l.trim() != "subject-path: |")
        .skip(1)
        .take_while(|l| !l.trim().is_empty() && !l.contains(':'))
        .map(|l| l.trim().replace(VERSION_EXPR, VERSION))
        .map(|p| p.rsplit('/').next().unwrap().to_owned())
        .collect()
}

/// Schlüssel direkt unter `on:` (Auslöser).
fn triggers(yml: &str) -> Vec<String> {
    yml.lines()
        .skip_while(|l| *l != "on:")
        .skip(1)
        .take_while(|l| l.is_empty() || l.starts_with(' '))
        .filter(|l| {
            l.starts_with("  ") && !l.starts_with("   ") && !l.trim_start().starts_with('#')
        })
        .map(|l| l.trim().trim_end_matches(':').to_owned())
        .collect()
}

#[test]
fn release_workflow_laeuft_nur_von_hand_auf_main_ohne_secrets_und_baut_das_produkt() {
    let yml = workflow();
    assert_eq!(triggers(&yml), ["workflow_dispatch"]);
    assert!(yml.contains(&format!("github.ref == '{GIT_REF}'")));
    assert!(yml.contains(&format!("github.repository == '{OWNER}/{REPO}'")));
    assert!(yml.contains("environment: release"));
    assert!(yml.contains("\npermissions: {}\n"));
    for bad in [
        "secrets.",
        "secrets[",
        "toJSON(secrets",
        "rust-cache",
        "actions/cache",
        "pull_request",
    ] {
        assert!(!yml.contains(bad), "{bad}");
    }
    // Jede Aktion per voller Commit-ID (40 Hex-Zeichen).
    let uses: Vec<&str> = yml
        .lines()
        .filter_map(|l| l.split_once("uses: ").map(|(_, u)| u))
        .collect();
    assert!(
        uses.iter()
            .any(|u| u.starts_with("actions/attest-build-provenance@"))
    );
    for u in uses {
        let sha = u
            .split_once('@')
            .unwrap()
            .1
            .split_whitespace()
            .next()
            .unwrap();
        assert!(
            sha.len() == 40 && sha.bytes().all(|b| b.is_ascii_hexdigit()),
            "{u}"
        );
    }
    assert!(yml.contains("--features produkt"));
    // Das Release trägt Setup, Eintrag und Bündel unter `v<Version>` (Pfad wie `release_path_prefix`).
    let create = yml
        .lines()
        .find(|l| l.contains("gh release create"))
        .unwrap();
    assert!(create.contains("\"v$env:VERSION\""), "{create}");
    for f in [ENTRY_FILE, BUNDLE_FILE] {
        assert!(yml.contains(&format!("release-dateien/{f}")), "{f}");
    }
    // Bezeugt werden genau Eintrag und Setup (der Prüfweg verlangt beide im selben Bündel).
    let setup = format!("daedyx-tasks-{VERSION}-setup.exe");
    assert_eq!(subjects(&yml), [ENTRY_FILE.to_owned(), setup]);
}

/// Ablauf mit dem Skript des Workflows (startet Windows PowerShell).
#[cfg(windows)]
#[path = "release_flow_tests.rs"]
mod flow;
