//! Ein erfundenes Release, gebaut wie im Workflow (`scripts/ci/make-entry.ps1`, Kennung, Dateinamen und Subjekte aus
//! `release.yml`), besteht die Selbstprüfung `release_check::verify`, also Suche und Download des Kerns mit der festen
//! Quelle dieser App. Nur Windows: Das Skript läuft mit Windows PowerShell 5.1, wie im Workflow-Schritt.
use super::{VERSION, repo_root, subjects, workflow, workflow_app};
use crate::release_check;
use crate::test_support::tmp;
use crate::update::{OWNER, REPO};
use daedyx_sys::{RunEnd, run_with_limit};
use daedyx_tasks_store::test_support::TestDir;
use daedyx_updater::daedyx_update::attest::{AttestError, AttestKind, Provenance};
use daedyx_updater::daedyx_update::check::{BUNDLE_FILE, ENTRY_FILE};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

const SETUP: &[u8] = b"erfundenes Produkt-Setup 9.9.9";
const BUNDLE: &[u8] = b"{\"erfunden\": true}";
const NOTES: &str = "Erfundene Neuerung mit \u{201e}Anf\u{fc}hrungszeichen\u{201c} und \"Zitat\".";

/// Nimmt nur die Subjekte an, die der Workflow attestiert, und nur im veröffentlichten Bündel.
struct Subjects(Vec<[u8; 32]>);

impl Provenance for Subjects {
    fn verify(&self, sha256: &[u8; 32], bundle: &[u8]) -> Result<(), AttestError> {
        let kind = if bundle != BUNDLE {
            AttestKind::Malformed
        } else if self.0.contains(sha256) {
            return Ok(());
        } else {
            AttestKind::NotInBundle
        };
        Err(AttestError {
            kind,
            detail: "Attrappe".into(),
        })
    }
}

fn sha(path: &Path) -> [u8; 32] {
    daedyx_sys::file_sha256(path).unwrap()
}

/// Text als PowerShell-Wort: Schalter wie `-App` bleiben, alles andere steht in einfachen Anführungszeichen.
fn ps_word(s: &str) -> String {
    let switch =
        s.len() > 1 && s.starts_with('-') && s.chars().skip(1).all(|c| c.is_ascii_alphabetic());
    if switch {
        s.to_owned()
    } else {
        format!("'{}'", s.replace('\'', "''"))
    }
}

/// Windows PowerShell 5.1 mit einem Skript und Argumenten, aufgerufen wie im Workflow-Schritt (`& skript …`). Alles,
/// was das Skript schreibt, landet in einer Protokolldatei und steht bei einem Fehler in der Testausgabe (Release-Lauf
/// 1 war auf dem Läufer rot ohne jeden Hinweis).
pub(super) fn powershell(script: &Path, args: &[&str]) -> PsRun {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let dir = tmp(&format!("ps-{}", RUNS.fetch_add(1, Ordering::Relaxed)));
    let log_file = dir.join("ausgabe.txt");
    let log = ps_word(log_file.to_str().unwrap());
    let call: Vec<String> = std::iter::once(script.to_str().unwrap())
        .chain(args.iter().copied())
        .map(ps_word)
        .collect();
    // Modulpfad wie in einem frischen Windows PowerShell: Läuft `cargo test` unter pwsh 7 (Release-Lauf), erbt das
    // Kind dessen Pfad mit den Modulen von PowerShell 7, die Windows PowerShell nicht laden kann.
    let command = format!(
        "$env:PSModulePath = [Environment]::GetEnvironmentVariable('PSModulePath', 'Machine'); \
         try {{ & {} *> {log}; exit $LASTEXITCODE }} catch {{ [IO.File]::AppendAllText({log}, ($_ | Out-String)); exit 1 }}",
        call.join(" ")
    );
    let root = std::env::var_os("SystemRoot").unwrap();
    let ps = Path::new(&root).join(r"System32\WindowsPowerShell\v1.0\powershell.exe");
    let head = [
        "-NoProfile",
        "-NonInteractive",
        "-ExecutionPolicy",
        "Bypass",
        "-Command",
    ];
    let all: Vec<&str> = head.into_iter().chain([command.as_str()]).collect();
    let end = run_with_limit(&ps, &all, Duration::from_secs(60));
    let log = std::fs::read_to_string(&log_file).unwrap_or_default();
    PsRun { end, log }
}

/// Ende eines Skripts mit seiner Ausgabe. Verglichen wird nur das Ende; die Ausgabe steht in der Meldung von
/// `assert_eq!`, wenn der Vergleich scheitert.
pub(super) struct PsRun {
    end: RunEnd,
    log: String,
}

impl PartialEq<RunEnd> for PsRun {
    fn eq(&self, other: &RunEnd) -> bool {
        self.end == *other
    }
}

impl std::fmt::Debug for PsRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, Ausgabe des Skripts:\n{}", self.end, self.log)
    }
}

/// `entry.json` mit dem Skript des Workflows; liefert das Ende des Skripts.
pub(super) fn make_entry(dir: &Path, setup: &Path, app: &str, notes: &str) -> PsRun {
    let notes_file = dir.join("notiz.txt");
    std::fs::write(&notes_file, notes).unwrap();
    let s = |p: &Path| p.to_str().unwrap().to_owned();
    let repo = format!("{OWNER}/{REPO}");
    let (notes_file, setup, out) = (s(&notes_file), s(setup), s(&dir.join(ENTRY_FILE)));
    let args = [
        "-App",
        app,
        "-Repo",
        &repo,
        "-Version",
        VERSION,
        "-Date",
        "2026-12-24",
        "-NotesFile",
        &notes_file,
        "-Setup",
        &setup,
        "-Out",
        &out,
    ];
    powershell(&repo_root().join(r"scripts\ci\make-entry.ps1"), &args)
}

/// Ordner mit Setup, `entry.json` und Bündel, wie ihn der Workflow vor der Selbstprüfung hat.
fn release_dir(name: &str, app: &str) -> TestDir {
    let setup_name = format!("daedyx-tasks-{VERSION}-setup.exe");
    assert_eq!(
        subjects(&workflow()),
        [ENTRY_FILE.to_owned(), setup_name.clone()]
    );
    let dir = tmp(name);
    let setup = dir.join(&setup_name);
    std::fs::write(&setup, SETUP).unwrap();
    assert_eq!(
        make_entry(&dir, &setup, app, NOTES),
        RunEnd::Exited(Some(0))
    );
    std::fs::write(dir.join(BUNDLE_FILE), BUNDLE).unwrap();
    std::fs::remove_file(dir.join("notiz.txt")).unwrap();
    dir
}

/// Kennung aus `release.yml`; die Vorschau prüft mit ihrer eigenen Regel und damit ihrer eigenen Kennung.
fn app_for_this_build() -> String {
    if crate::ids::PREVIEW {
        crate::ids::UPDATE_APP.to_owned()
    } else {
        workflow_app(&workflow())
    }
}

#[test]
fn erfundenes_release_wie_der_workflow_es_baut_besteht_die_selbstpruefung() {
    let dir = release_dir("t7v-release", &app_for_this_build());
    let setup = dir.join(format!("daedyx-tasks-{VERSION}-setup.exe"));
    let all = Subjects(vec![sha(&dir.join(ENTRY_FILE)), sha(&setup)]);
    let got = release_check::verify(&dir, VERSION, &all, &dir.join("probe")).unwrap();
    assert_eq!(std::fs::read(got).unwrap(), SETUP);
    // Gegenproben: ohne den Eintrag unter den Subjekten, mit falscher Version, mit falscher Kennung.
    let only_setup = Subjects(vec![sha(&setup)]);
    assert!(release_check::verify(&dir, VERSION, &only_setup, &dir.join("p2")).is_err());
    assert!(release_check::verify(&dir, "9.9.8", &all, &dir.join("p3")).is_err());
    let other = release_dir("t7v-falsch", "daedyx-tasks-falsch");
    let all = Subjects(vec![sha(&other.join(ENTRY_FILE)), sha(&setup)]);
    assert!(release_check::verify(&other, VERSION, &all, &other.join("p4")).is_err());
}
