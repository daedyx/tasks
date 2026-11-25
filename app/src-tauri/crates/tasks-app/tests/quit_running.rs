//! Review T5, M5-1: `--quit-running` schickt nur die Nachricht an eine laufende Vorschau (oder endet, wenn keine
//! läuft) und legt dabei keinen Datenordner, keine Datenbank und keinen Schlüssel an. `APPDATA` zeigt nur für den
//! Kindprozess auf einen Wegwerf-Ordner, ebenso `USERPROFILE`: So hat er eine eigene Pipe und beendet keine laufende Vorschau.
//! Nur die Vorschau-Fassung: Die Produkt-Fassung startet dieser Test nie (`cfg!` statt `#[cfg]`, wie `kennung.rs`).
#![cfg(windows)]
use std::process::Command;
use std::time::{Duration, Instant};

const EXE: &str = env!("CARGO_BIN_EXE_daedyx-tasks-vorschau");

#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "SICHERHEIT-AUSNAHMEN A-033: Kindprozess mit eigenem APPDATA, das kann daedyx-sys::run_with_limit nicht"
)]
fn beenden_einer_laufenden_vorschau_legt_keine_daten_an() {
    if cfg!(feature = "produkt") {
        return;
    }
    let base = std::env::temp_dir().join(format!("daedyx-tasks-test-quit-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let mut child = Command::new(EXE)
        .arg("--quit-running")
        .env("APPDATA", &base)
        // Eigene Pipe (der Name hängt am Benutzerprofil): Eine laufende Vorschau des Benutzers bleibt unberührt, und
        // fiele M5-1 je zurück, entstünde nichts außerhalb des Wegwerf-Ordners (Review T5, H5-5).
        .env("USERPROFILE", &base)
        .spawn()
        .unwrap();
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait().unwrap() {
            break s;
        }
        if start.elapsed() > Duration::from_secs(15) {
            let _ = child.kill();
            panic!("hängt");
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    assert_eq!(status.code(), Some(0));
    let left: Vec<_> = std::fs::read_dir(&base)
        .unwrap()
        .flatten()
        .map(|e| e.file_name())
        .collect();
    assert!(left.is_empty(), "{left:?}");
    let _ = std::fs::remove_dir_all(&base);
}
