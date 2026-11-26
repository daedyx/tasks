//! Update in der App (Auftrag U2, plan-update-bibliothek): feste Identität der Update-Quelle, Start der Suche mit dem
//! Datenordner dieser Fassung, Installation nach Klick. Vertrauen entsteht nur aus dem GitHub-Nachweis und dieser
//! Identität (Besitzer, Repo, numerische Kennungen; Workflow `release.yml` auf `main` legt der Kern fest), kein
//! Schlüssel, kein Token. Ohne Netz bleibt es still („später erneut“). Installiert wird nie ohne Klick.
use crate::app::TasksApp;
use crate::ids;
use crate::paths::{self, DataDir};
use daedyx_sys::StartError;
use daedyx_updater::daedyx_update::policy::{Platform, UpdatePolicy};
use daedyx_updater::daedyx_update::version::Version;
use daedyx_updater::install::Starter;
use daedyx_updater::{Backend, Config, Prepared, Status, Updater};
use eframe::egui::Context;
use std::path::PathBuf;

/// Besitzer und Repo der Releases (`github.com/daedyx/tasks`).
pub const OWNER: &str = "daedyx";
pub const REPO: &str = "tasks";
/// Numerische GitHub-Kennungen von Besitzer und Repo, am 2026-11-26 einmal mit `gh api repos/daedyx/tasks` gelesen
/// (nur lesen). Sie schützen davor, dass jemand den Namen nach Löschen oder Umbenennen neu anlegt (Spike P4).
pub const OWNER_ID: u64 = 356_409_479;
pub const REPO_ID: u64 = 1_464_583_596;
/// Unterordner des Datenordners für geladene Setups (wie `daedyx_updater::Config::new`).
pub const UPDATE_DIR: &str = "updates";

/// Regel der Update-Quelle für diese Fassung und Plattform (`None` auf nicht unterstützten Systemen).
pub fn policy() -> Option<UpdatePolicy> {
    UpdatePolicy::new(
        OWNER,
        REPO,
        OWNER_ID,
        REPO_ID,
        ids::UPDATE_APP,
        Platform::current()?,
    )
    .ok()
}

/// Laufende Version.
pub fn installed() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap_or(Version::new(0, 0, 0))
}

/// Angaben für den Ablauf: Datenordner dieser Fassung, gemeinsame Sperre mit dem Launcher unter `local`.
pub fn config(data: &DataDir, local: &std::path::Path) -> Option<Config> {
    Some(Config::new(policy()?, installed(), data.path(), local))
}

/// Startet den Ablauf mit echtem Netz und echtem Nachweis.
pub fn start(ctx: &Context, data: &DataDir) -> Updater {
    match paths::local_base().and_then(|local| config(data, &local)) {
        Some(c) => Updater::start(c, Backend::real(), Some(ctx.clone())),
        None => Updater::disabled(),
    }
}

impl TasksApp {
    /// Geprüftes Setup starten (plan-update-bibliothek 6): erst den Entwurf im Detailbereich sichern (sonst kein
    /// Update), dann das Setup starten und schließen wie „Beenden“ im Infobereich. Das Setup wartet auf das Ende.
    pub fn install_update(&mut self, prepared: Prepared) {
        self.install_update_with(prepared, &daedyx_sys::start_verified);
    }

    /// Wie [`TasksApp::install_update`] mit eigenem Starter (Tests).
    pub fn install_update_with(&mut self, prepared: Prepared, starter: Starter<'_>) {
        if !self.tasks.commit_draft(&self.texts) {
            // Das Setup bleibt im Update-Ordner und wird beim nächsten Start aufgeräumt.
            self.updater.set_status(Status::NotSaved);
            return;
        }
        let started = if daedyx_updater::install::supported() {
            prepared.start_with(self.bg.window.hidden, starter)
        } else {
            Err(StartError::Failed)
        };
        match started {
            Ok(()) => {
                self.updater.set_status(Status::Installing);
                self.bg.window.quit_wanted = true;
            }
            Err(_) => self.updater.set_status(Status::StartFailed),
        }
    }

    /// Installationsprobe der Vorschau (`probe.rs`): lokales Setup ohne Nachweis, sonst wie ein Update.
    pub(crate) fn run_update_probe(&mut self, setup: PathBuf) {
        let args = daedyx_updater::install::setup_args(self.bg.window.hidden);
        let started = daedyx_sys::file_sha256(&setup)
            .ok_or(StartError::Missing)
            .and_then(|sha| daedyx_sys::start_verified(&setup, &sha, &args));
        if started.is_ok() && self.tasks.commit_draft(&self.texts) {
            self.updater.set_status(Status::Installing);
            self.bg.window.quit_wanted = true;
        }
    }
}
