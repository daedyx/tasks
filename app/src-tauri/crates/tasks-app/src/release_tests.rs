//! T7-Vorbereitung (F-U2-2, Review T7v): Der Release-Workflow passt zum Prüfweg des Kerns. Er liegt genau unter
//! `policy::WORKFLOW`, läuft nur auf `policy::GIT_REF` im eigenen Repo, braucht kein Secret, baut Programm und Setup
//! der Produkt-Fassung und prüft vor dem Veröffentlichen mit dem Prüfweg der App (`release_check`). Ein erfundenes
//! Release, wie der Workflow es baut (`scripts/ci/make-entry.ps1`, Kennung, Dateinamen und Subjekte aus `release.yml`),
//! besteht diesen Prüfweg. Die Sigstore-Kette selbst prüft der Kern gegen ein echtes Bündel derselben Form
//! (`release.yml@refs/heads/main`, astral-sh/uv); hier steht eine Attrappe, die genau die Subjekte kennt.
use crate::update::{OWNER, REPO};
use daedyx_updater::daedyx_update::check::{BUNDLE_FILE, ENTRY_FILE};
use daedyx_updater::daedyx_update::policy::{GIT_REF, WORKFLOW};
use std::path::{Path, PathBuf};

const VERSION: &str = "9.9.9";
/// So setzt der Workflow die Version in Dateinamen ein.
const VERSION_EXPR: &str = "${{ needs.bauen.outputs.version }}";
/// App-Kennung der Produkt-Fassung (`ids::UPDATE_APP` mit `produkt`; die Vorschau hat sie nicht im Programm).
const PRODUCT_APP: &str = "daedyx-tasks";
/// Muster des Secret-Wächters für `release.yml`, wörtlich wie in `ci.yml` (M-T7v-1: eine Regel, zwei Orte).
const RELEASE_SECRET_LINE: &str = r"$releaseMuster = 'secrets\s*[.\[:)]', '\(\s*secrets'";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

/// Datei aus dem Repo mit LF-Zeilenenden (Git legt sie unter Windows mit CRLF ab).
fn repo_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap()
        .replace("\r\n", "\n")
}

fn workflow() -> String {
    repo_text(WORKFLOW)
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

/// App-Kennung, die der Workflow an `make-entry.ps1` gibt.
fn workflow_app(yml: &str) -> String {
    let line = yml
        .lines()
        .find(|l| l.contains("make-entry.ps1 -App "))
        .unwrap();
    let rest = line.split_once("-App ").unwrap().1;
    rest.split_whitespace().next().unwrap().to_owned()
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

/// Zeilen (ab 1), in denen die Muster aus [`RELEASE_SECRET_LINE`] anschlagen: `secrets`, dahinter `.`, `[`, `:` oder
/// `)`, oder davor `(`, jeweils mit Leerraum dazwischen. Groß/klein zählt, wie `Select-String -CaseSensitive`.
fn secret_hits(text: &str) -> Vec<usize> {
    let hit = |l: &str| {
        l.match_indices("secrets").any(|(i, w)| {
            let (head, tail) = l.split_at(i);
            let after = tail.strip_prefix(w).unwrap().trim_start().chars().next();
            let before = head.trim_end().chars().last();
            matches!(after, Some('.' | '[' | ':' | ')')) || before == Some('(')
        })
    };
    (1..)
        .zip(text.lines())
        .filter(|(_, l)| hit(l))
        .map(|(n, _)| n)
        .collect()
}

/// Beispiele für den Secret-Wächter: jede Zeile ein Zugriff; dazu harmlose Wörter (`HARMLESS`).
const SECRET_SAMPLES: &str = "x: ${{ secrets.TOKEN }}\nx: ${{ secrets['TOKEN'] }}\nx: ${{ toJSON(secrets) }}\n\
x: ${{ toJSON( secrets ) }}\n    secrets: inherit\nif: secrets.X != ''\nx: ${{ secrets . X }}\n";
const HARMLESS: &str = "# plan-ohne-secrets Schritt 6\nkeine secrets im Lauf\nSecrets: keine\n";

#[test]
fn secret_waechter_findet_zugriffe_aber_nicht_das_wort() {
    assert_eq!(secret_hits(SECRET_SAMPLES), (1..=7).collect::<Vec<_>>());
    assert_eq!(secret_hits(HARMLESS), Vec::<usize>::new());
    let ci = repo_text(".github/workflows/ci.yml");
    assert!(ci.lines().any(|l| l.trim() == RELEASE_SECRET_LINE));
    assert!(
        ci.contains("-Path .github/workflows/release.yml -CaseSensitive -Pattern $releaseMuster")
    );
    assert_eq!(secret_hits(&workflow()), Vec::<usize>::new());
}

#[test]
fn release_workflow_laeuft_nur_von_hand_auf_main_ohne_secrets_und_baut_das_produkt() {
    let yml = workflow();
    assert_eq!(triggers(&yml), ["workflow_dispatch"]);
    assert!(yml.contains(&format!("github.ref == '{GIT_REF}'")));
    assert!(yml.contains(&format!("github.repository == '{OWNER}/{REPO}'")));
    assert!(yml.contains("environment: release"));
    assert!(yml.contains("\npermissions: {}\n"));
    // Fester Läufer statt eines wandernden `-latest` (H-T7v-2); welcher je Job, prüft `jobs`.
    assert!(!yml.contains("windows-latest"), "{yml}");
    for bad in ["rust-cache", "actions/cache", "pull_request", "vorschau"] {
        assert!(!yml.contains(bad), "{bad}");
    }
    // Jede Aktion per voller Commit-ID (40 Hex-Zeichen, nicht der Platzhalter aus Nullen, H-T7v-3).
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
            sha.len() == 40
                && sha.bytes().all(|b| b.is_ascii_hexdigit())
                && sha.bytes().any(|b| b != b'0'),
            "{u}"
        );
    }
    // Produkt-Fassung: Programm, Setup-Vorlage und Programmname (N-T7v-2), Kennung im Eintrag (N-T7v-1).
    assert!(yml.contains(
        "cargo build --release --locked --offline -p daedyx-tasks-app --features produkt"
    ));
    assert!(yml.contains("$nsi = 'installer/tasks.nsi'"));
    assert!(yml.contains("-Exe daedyx-tasks.exe "));
    assert_eq!(workflow_app(&yml), PRODUCT_APP);
    if !crate::ids::PREVIEW {
        assert_eq!(PRODUCT_APP, crate::ids::UPDATE_APP);
    }
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

/// Vier Jobs: Bauen, Bezeugen, Prüfen, Veröffentlichen, je mit eigenen Rechten (N-T7v-4, M-T7v-3).
#[path = "release_jobs_tests.rs"]
mod jobs;

/// Ablauf mit dem Skript des Workflows (startet Windows PowerShell).
#[cfg(windows)]
#[path = "release_flow_tests.rs"]
mod flow;

/// Secret-Wächter und Notiz-Regeln, mit Windows PowerShell gegengeprüft.
#[cfg(windows)]
#[path = "release_guard_tests.rs"]
mod guard;
