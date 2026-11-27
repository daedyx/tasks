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

/// Windows PowerShell 5.1 mit einem Skript und Argumenten.
pub(super) fn powershell(script: &Path, args: &[&str]) -> RunEnd {
    let root = std::env::var_os("SystemRoot").unwrap();
    let ps = Path::new(&root).join(r"System32\WindowsPowerShell\v1.0\powershell.exe");
    let script = script.to_str().unwrap();
    let head = [
        "-NoProfile",
        "-NonInteractive",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        script,
    ];
    let all: Vec<&str> = head.iter().chain(args).copied().collect();
    run_with_limit(&ps, &all, std::time::Duration::from_secs(60))
}

/// `entry.json` mit dem Skript des Workflows; liefert das Ende des Skripts.
pub(super) fn make_entry(dir: &Path, setup: &Path, app: &str, notes: &str) -> RunEnd {
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
