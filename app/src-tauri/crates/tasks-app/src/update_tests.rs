//! Update in Tasks (U2) ohne Netz: feste Quelle, geprüftes Setup startet erst nach Klick und nur, wenn der Entwurf
//! gesichert ist; danach beendet sich die App wie „Beenden“; versteckt bleibt versteckt. Gefälschter Server und
//! Attrappe für den Nachweis, erfundenes Release 9.9.9, Wegwerf-Ordner.
use crate::settings::Settings;
use crate::test_support::{app, tmp};
use crate::update::{OWNER, OWNER_ID, REPO, REPO_ID, config, installed, policy};
use daedyx_sys::StartError;
use daedyx_updater::daedyx_update::attest::{AttestError, Provenance};
use daedyx_updater::daedyx_update::fetch::{HttpsUrl, Reply, Transport, TransportError};
use daedyx_updater::{Backend, Status, Updater};
use eframe::egui::Context;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::mpsc::{Sender, channel};
use std::time::{Duration, Instant};

const SETUP: &[u8] = b"erfundenes Setup 9.9.9";

fn sha(b: &[u8]) -> [u8; 32] {
    let dir = tmp(&format!("u2-pruefsumme-{:?}", std::thread::current().id()));
    let file = dir.join("setup.exe");
    std::fs::write(&file, b).unwrap();
    daedyx_sys::file_sha256(&file).unwrap()
}

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

/// Nimmt alles an: Die Prüfung selbst testet der Kern; hier geht es um den Ablauf in der App.
struct Trusting;

impl Provenance for Trusting {
    fn verify(&self, _: &[u8; 32], _: &[u8]) -> Result<(), AttestError> {
        Ok(())
    }
}

fn backend() -> Backend {
    let p = policy().unwrap();
    let url = format!("https://github.com/{OWNER}/{REPO}/releases/download/v9.9.9/setup.exe");
    let entry = serde_json::to_vec(&serde_json::json!({
        "schema": 1, "app": crate::ids::UPDATE_APP, "version": "9.9.9", "date": "2026-12-24",
        "notes": "Erfundene Neuerung.",
        "packages": {(p.platform().key()): {"url": url.clone(), "sha256": hex_of(&sha(SETUP)),
            "size": SETUP.len(), "kind": "nsis-silent"}}
    }))
    .unwrap();
    let routes = BTreeMap::from([
        (p.latest_asset_url("entry.json"), entry),
        (
            p.latest_asset_url("provenance.sigstore.json"),
            b"{}".to_vec(),
        ),
        (url, SETUP.to_vec()),
    ]);
    Backend {
        transport: Box::new(Server(routes)),
        provenance: Box::new(|_| Ok(Box::new(Trusting) as Box<dyn Provenance>)),
        free_space: Box::new(|_: &Path| None),
        now: Box::new(|| 1_800_000_000),
    }
}

fn hex_of(b: &[u8; 32]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn until(u: &mut Updater, done: impl Fn(&Status) -> bool) {
    let start = Instant::now();
    while !done(u.status()) {
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "{:?}",
            u.status()
        );
        std::thread::sleep(Duration::from_millis(5));
        u.poll();
    }
}

/// App mit einem geladenen, geprüften Setup (Ablauf bis „Jetzt aktualisieren“).
fn ready(
    name: &str,
) -> (
    crate::app::TasksApp,
    daedyx_updater::Prepared,
    daedyx_tasks_store::test_support::TestDir,
) {
    let base = tmp(name);
    let dir = crate::paths::data_dir_in(&base).unwrap();
    let mut c = config(&dir, &base).unwrap();
    c.auto_delay = Duration::ZERO;
    let mut a = app(&Settings::default());
    a.updater = Updater::start(c, backend(), None);
    a.updater.check_now();
    until(&mut a.updater, |s| {
        matches!(s, Status::Available { .. } | Status::Blocked(_))
    });
    assert!(matches!(a.updater.status(), Status::Available { version, .. } if version == "9.9.9"));
    a.updater.update_now();
    until(&mut a.updater, |s| {
        matches!(s, Status::Ready { .. } | Status::Blocked(_))
    });
    let p = a.updater.take_ready().unwrap();
    (a, p, base)
}

/// Starter, der nur die Argumente meldet.
type Recorder = Box<dyn Fn(&Path, &[u8; 32], &[&str]) -> Result<(), StartError>>;

fn recorder() -> (Recorder, std::sync::mpsc::Receiver<String>) {
    let (tx, rx): (Sender<String>, _) = channel();
    let starter = move |_: &Path, _: &[u8; 32], a: &[&str]| {
        tx.send(a.join(" ")).map_err(|_| StartError::Failed)
    };
    (Box::new(starter), rx)
}

#[test]
fn quelle_und_kennung_stehen_fest() {
    let p = policy().unwrap();
    assert_eq!(
        (p.owner(), p.repo(), p.owner_id(), p.repo_id()),
        ("daedyx", "tasks", OWNER_ID, REPO_ID)
    );
    assert_eq!(
        p.app_id(),
        crate::ids::VAULT,
        "Eintrag der anderen Fassung wird abgelehnt"
    );
    assert_eq!(installed().to_string(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn geprueftes_setup_startet_und_die_app_beendet_sich() {
    let (mut a, p, _base) = ready("u2-start");
    let (starter, rx) = recorder();
    a.install_update_with(p, &starter);
    assert_eq!(rx.try_recv().unwrap(), "/P /UPDATE /R");
    assert_eq!(a.updater.status(), &Status::Installing);
    a.logic(&Context::default());
    assert!(a.bg.window.quitting, "beendet wie „Beenden“ im Infobereich");
}

#[test]
fn versteckte_app_startet_nach_dem_update_versteckt() {
    let (mut a, p, _base) = ready("u2-versteckt");
    a.bg.window.start_hidden();
    let (starter, rx) = recorder();
    a.install_update_with(p, &starter);
    assert_eq!(rx.try_recv().unwrap(), "/P /UPDATE /R /ARGS --hidden");
}

#[test]
fn ohne_gesicherten_entwurf_kein_update() {
    let (mut a, p, _base) = ready("u2-entwurf");
    let t = a.tasks.items.first().cloned().unwrap();
    a.tasks.draft.id = t.id.clone();
    a.tasks.draft.title = t.title.clone();
    a.tasks.draft.notes = t.notes.clone();
    a.tasks.draft.date = "31.02.2026".into();
    let (starter, rx) = recorder();
    a.install_update_with(p, &starter);
    assert!(rx.try_recv().is_err(), "kein Setup gestartet");
    assert_eq!(a.updater.status(), &Status::NotSaved);
    assert!(!a.bg.window.quit_wanted && !a.bg.window.quitting);
}

#[test]
fn scheitert_der_start_bleibt_die_app_offen() {
    let (mut a, p, _base) = ready("u2-startfehler");
    let fail = |_: &Path, _: &[u8; 32], _: &[&str]| Err(StartError::Failed);
    a.install_update_with(p, &fail);
    assert_eq!(a.updater.status(), &Status::StartFailed);
    a.logic(&Context::default());
    assert!(!a.bg.window.quitting);
}
