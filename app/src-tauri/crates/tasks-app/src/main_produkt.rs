#![cfg_attr(not(test), windows_subsystem = "windows")]
//! Daedyx Tasks, Produkt-Fassung (Programm `daedyx-tasks.exe`, nur mit dem Merkmal `produkt`; T7, Lücke L3).
//! Derselbe Start wie das Vorschau-Programm (`main.rs`); welche Kennungen gelten, entscheidet `ids.rs`.
use daedyx_background::args::Args;
use daedyx_tasks_app::launch;
use daedyx_tasks_app::settings::Settings;
use std::process::ExitCode;

fn main() -> ExitCode {
    launch::run(Settings::from_env(), Args::parse(std::env::args().skip(1)))
}
