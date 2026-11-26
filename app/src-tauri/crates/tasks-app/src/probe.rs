//! Nur in der Vorschau übersetzt: Installationsprobe des Updates (plan-update-bibliothek 10, Muster M8). Die Probe
//! setzt `DAEDYX_UPDATE_PROBE=<Setup>` und `DAEDYX_UPDATE_PROBE_VON=<installierte Version>`; die laufende App startet
//! dann dieses Setup wie ein geprüftes Update (`/P /UPDATE /R`, versteckt mit `/ARGS --hidden`) und beendet sich. Den
//! Nachweis braucht die Probe nicht, sie prüft nur Start, Ende und Installer. Damit die neu gestartete App (gleiche
//! Umgebung) nicht noch einmal startet, gilt es nur, solange `app.json` neben dem Programm genau die Ausgangsversion
//! nennt. Die Produkt-Fassung enthält diese Datei nicht, dort steht `probe_off.rs` (Ausnahme A-034 in
//! `docs/SICHERHEIT-AUSNAHMEN.md`).
use crate::app::TasksApp;
use daedyx_sys::StartError;
use daedyx_updater::Status;
use daedyx_updater::install::{Starter, setup_args};
use std::path::{Path, PathBuf};

pub const SETUP_VAR: &str = "DAEDYX_UPDATE_PROBE";
pub const FROM_VAR: &str = "DAEDYX_UPDATE_PROBE_VON";

/// Setup der Probe, wenn sie für das Programm in `exe_dir` gilt.
pub fn setup_for(exe_dir: &Path, setup: Option<String>, from: Option<String>) -> Option<PathBuf> {
    let (setup, from) = (setup.filter(|s| !s.is_empty())?, from?);
    let json = std::fs::read(exe_dir.join("app.json")).ok()?;
    let value: serde_json::Value = serde_json::from_slice(&json).ok()?;
    (value.get("version")?.as_str()? == from).then(|| PathBuf::from(setup))
}

/// Läuft die App in der Umgebung einer Probe? Dann startet keine echte Update-Suche (kein Netz), auch nicht in der
/// nach dem Update neu gestarteten App, für die [`setup`] nichts mehr liefert.
pub fn active() -> bool {
    std::env::var_os(SETUP_VAR).is_some_and(|v| !v.is_empty())
}

/// Setup der Probe aus der Umgebung und dem Ordner des laufenden Programms.
pub fn setup() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    setup_for(
        exe.parent()?,
        std::env::var(SETUP_VAR).ok(),
        std::env::var(FROM_VAR).ok(),
    )
}

/// Lage der Probe in der laufenden App.
#[derive(Debug, Default)]
pub struct Probe {
    /// Setup, das beim nächsten Bild startet (einmal).
    setup: Option<PathBuf>,
    /// Umgebung einer Probe: keine echte Update-Suche.
    quiet: bool,
}

impl Probe {
    /// Aus der Umgebung; gesetzt nur an der einen Stelle in `launch.rs` (A-035).
    pub fn from_env() -> Probe {
        Probe {
            setup: setup(),
            quiet: active(),
        }
    }

    pub fn quiet(&self) -> bool {
        self.quiet
    }
}

impl TasksApp {
    /// Startet das Setup der Probe, falls eines ansteht (jedes Bild aus `logic`).
    pub(crate) fn run_update_probe(&mut self) {
        if let Some(setup) = self.probe.setup.take() {
            self.run_update_probe_with(setup, &daedyx_sys::start_verified);
        }
    }

    /// Wie ein Update, nur ohne Nachweis: erst den Entwurf sichern (sonst nichts starten), dann das Setup über einen
    /// gesperrten Griff mit seiner Prüfsumme starten und schließen wie „Beenden“ (Review U2-R N-U2-1).
    pub(crate) fn run_update_probe_with(&mut self, setup: PathBuf, starter: Starter<'_>) {
        if !self.tasks.commit_draft(&self.texts) {
            self.updater.set_status(Status::NotSaved);
            return;
        }
        let args = setup_args(self.bg.window.hidden);
        let started = daedyx_sys::file_sha256(&setup)
            .ok_or(StartError::Missing)
            .and_then(|sha| starter(&setup, &sha, &args));
        match started {
            Ok(()) => {
                self.updater.set_status(Status::Installing);
                self.bg.window.quit_wanted = true;
            }
            Err(_) => self.updater.set_status(Status::StartFailed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::setup_for;
    use crate::test_support::tmp;

    fn some(s: &str) -> Option<String> {
        Some(s.to_owned())
    }

    #[test]
    fn probe_gilt_nur_fuer_die_ausgangsversion_und_nie_doppelt() {
        let dir = tmp("u2-probe");
        let setup = some("C:/probe/setup.exe");
        // Ohne app.json neben dem Programm nie.
        assert_eq!(setup_for(&dir, setup.clone(), some("0.0.1")), None);
        std::fs::write(
            dir.join("app.json"),
            br#"{"version": "0.0.1", "exe": "x.exe"}"#,
        )
        .unwrap();
        assert!(setup_for(&dir, setup.clone(), some("0.0.1")).is_some());
        assert_eq!(setup_for(&dir, setup.clone(), None), None);
        assert_eq!(setup_for(&dir, some(""), some("0.0.1")), None);
        assert_eq!(setup_for(&dir, None, some("0.0.1")), None);
        // Nach dem Update nennt app.json die neue Version: Die neu gestartete App startet nichts mehr.
        std::fs::write(
            dir.join("app.json"),
            br#"{"version": "0.0.2", "exe": "x.exe"}"#,
        )
        .unwrap();
        assert_eq!(setup_for(&dir, setup, some("0.0.1")), None);
    }
}

#[cfg(test)]
#[path = "probe_tests.rs"]
mod probe_tests;
