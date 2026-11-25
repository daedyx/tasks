#![cfg_attr(not(test), windows_subsystem = "windows")]
//! Daedyx Tasks – Vorschau der Rust-Fassung, nur mit erfundenen Daten.
//!
//! Befehlszeile: `--start-probe` (laden und enden), `--hidden` (versteckt starten, Autostart), `--quit-running`
//! (laufende Vorschau beenden), `--cleanup` (Autostart-Wert, App-Kennung und eigene Benachrichtigungen entfernen),
//! `--probe-toast` (Benachrichtigung mit erfundenem Inhalt zeigen und prüfen), `--takt <Sekunden>` (5–3600).
//! Umgebungsvariablen: siehe `settings.rs` und `DAEDYX_SCREENSHOT=datei.png` (Foto nur vom App-Fenster).
use daedyx_background::args::Args;
use daedyx_tasks_app::launch;
use daedyx_tasks_app::settings::Settings;
use std::process::ExitCode;

fn main() -> ExitCode {
    launch::run(Settings::from_env(), Args::parse(std::env::args().skip(1)))
}
