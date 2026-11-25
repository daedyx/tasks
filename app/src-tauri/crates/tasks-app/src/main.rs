#![cfg_attr(not(test), windows_subsystem = "windows")]
//! Daedyx Tasks – Vorschau der Rust-Fassung, nur mit erfundenen Daten. Befehlszeile: `--start-probe`.
//! Umgebungsvariablen: siehe `settings.rs` und `DAEDYX_SCREENSHOT=datei.png` (Foto nur vom App-Fenster).
use daedyx_tasks_app::launch;
use daedyx_tasks_app::settings::Settings;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    launch::run(Settings::from_env(), &args)
}
