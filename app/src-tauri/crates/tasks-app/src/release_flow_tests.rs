//! Ein erfundenes Release, gebaut wie im Workflow (`scripts/ci/make-entry.ps1`, Dateinamen und Subjekte aus
//! `release.yml`), besteht Suche und Download des Kerns mit der festen Quelle dieser App. Nur Windows: Das Skript
//! läuft mit Windows PowerShell 5.1, wie es auf jedem Windows liegt.
use super::{VERSION, repo_root, subjects, workflow};
use crate::test_support::tmp;
use crate::update::{OWNER, REPO, installed, policy};
use daedyx_updater::daedyx_update::attest::{AttestError, AttestKind, Provenance};
use daedyx_updater::daedyx_update::check::{self, BUNDLE_FILE, ENTRY_FILE, Outcome};
use daedyx_updater::daedyx_update::fetch::{HttpsUrl, Reply, Transport, TransportError};
use daedyx_updater::daedyx_update::version::Version;
use std::collections::BTreeMap;
use std::path::Path;

const SETUP: &[u8] = b"erfundenes Produkt-Setup 9.9.9";
const BUNDLE: &[u8] = b"{\"erfunden\": true}";
const NOTES: &str = "Erfundene Neuerung mit \u{201e}Anf\u{fc}hrungszeichen\u{201c} und \"Zitat\".";

struct Server(BTreeMap<String, Vec<u8>>);

impl Transport for Server {
    fn get(&mut self, url: &HttpsUrl) -> Result<Reply<'_>, TransportError> {
        Ok(match self.0.get(url.as_str()).cloned() {
            Some(b) => Reply::Body {
                length: Some(b.len() as u64),
                reader: Box::new(std::io::Cursor::new(b)),
            },
            None => Reply::Status(404),
        })
    }
}

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

fn sha(dir: &Path, bytes: &[u8]) -> [u8; 32] {
    let file = dir.join("pruefsumme.bin");
    std::fs::write(&file, bytes).unwrap();
    daedyx_sys::file_sha256(&file).unwrap()
}

/// `entry.json` mit dem Skript des Workflows (Windows PowerShell 5.1, wie es auf jedem Windows liegt).
fn make_entry(dir: &Path, setup: &Path, app: &str) -> Vec<u8> {
    use daedyx_sys::{RunEnd, run_with_limit};
    let notes = dir.join("notiz.txt");
    std::fs::write(&notes, NOTES).unwrap();
    let out = dir.join(ENTRY_FILE);
    let root = std::env::var_os("SystemRoot").unwrap();
    let ps = Path::new(&root).join(r"System32\WindowsPowerShell\v1.0\powershell.exe");
    let script = repo_root().join(r"scripts\ci\make-entry.ps1");
    let s = |p: &Path| p.to_str().unwrap().to_owned();
    let repo = format!("{OWNER}/{REPO}");
    let args = [
        "-NoProfile",
        "-NonInteractive",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        &s(&script),
        "-App",
        app,
        "-Repo",
        &repo,
        "-Version",
        VERSION,
        "-Date",
        "2026-12-24",
        "-NotesFile",
        &s(&notes),
        "-Setup",
        &s(setup),
        "-Out",
        &s(&out),
    ];
    let end = run_with_limit(&ps, &args, std::time::Duration::from_secs(60));
    assert_eq!(end, RunEnd::Exited(Some(0)));
    std::fs::read(out).unwrap()
}

#[test]
fn erfundenes_release_wie_der_workflow_es_baut_besteht_suche_und_download() {
    let setup_name = format!("daedyx-tasks-{VERSION}-setup.exe");
    assert_eq!(
        subjects(&workflow()),
        [ENTRY_FILE.to_owned(), setup_name.clone()]
    );
    let dir = tmp("t7v-release");
    let setup = dir.join(&setup_name);
    std::fs::write(&setup, SETUP).unwrap();
    let entry = make_entry(&dir, &setup, crate::ids::UPDATE_APP);
    let p = policy().unwrap();
    let url =
        format!("https://github.com/{OWNER}/{REPO}/releases/download/v{VERSION}/{setup_name}");
    let mut server = Server(BTreeMap::from([
        (p.latest_asset_url(ENTRY_FILE), entry.clone()),
        (p.latest_asset_url(BUNDLE_FILE), BUNDLE.to_vec()),
        (url, SETUP.to_vec()),
    ]));
    let all = Subjects(vec![sha(&dir, &entry), sha(&dir, SETUP)]);
    let Outcome::Available(offer) = check::check(&mut server, &all, &p, &installed(), None) else {
        panic!("kein Angebot");
    };
    assert_eq!(offer.entry().version, Version::parse(VERSION).unwrap());
    assert_eq!(offer.entry().notes, NOTES);
    let updates = dir.join("updates");
    std::fs::create_dir_all(&updates).unwrap();
    let got = check::fetch_setup(
        &mut server,
        &all,
        &p,
        &offer,
        &updates,
        None,
        &mut |_, _| {},
    )
    .unwrap();
    assert_eq!(std::fs::read(got).unwrap(), SETUP);
    // Gegenprobe: Ohne den Eintrag unter den Subjekten gibt es kein Angebot.
    let only_setup = Subjects(vec![sha(&dir, SETUP)]);
    assert!(matches!(
        check::check(&mut server, &only_setup, &p, &installed(), None),
        Outcome::Blocked(_)
    ));
}
