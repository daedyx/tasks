//! Testrahmen: App mit Speicher-Datenbank, erfundenen Aufgaben und fester Uhr (18.11.2026, 10:00, Berlin), Oberfläche
//! ohne Fenster (kittest) mit Tasks-Akzent.
use crate::app::TasksApp;
use crate::seed;
use crate::settings::Settings;
use chrono::NaiveDate;
use daedyx_tasks_ui::Clock;
use daedyx_ui::theme;
use daedyx_ui::tokens::AppAccent;
use egui_kittest::Harness;

pub fn clock() -> Clock {
    Clock::Fixed(
        NaiveDate::from_ymd_opt(2026, 11, 18)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(),
    )
}

pub fn app(settings: &Settings) -> TasksApp {
    TasksApp::new(settings, seed::demo(chrono_tz::Europe::Berlin, clock()))
}

/// Oberfläche mit `width` Pixeln Breite.
pub fn harness(app: TasksApp, width: f32) -> Harness<'static, TasksApp> {
    let mut h = Harness::builder()
        .with_size((width, 800.0))
        .with_context_setup(|ctx| theme::install(ctx, AppAccent::Tasks))
        .build_ui_state(|ui, app: &mut TasksApp| app.frame(ui), app);
    h.run();
    h
}

/// Frischer Wegwerf-Ordner im System-Temp (über den Store, Ausnahme A-005); nie ein Datenordner einer App.
pub fn tmp(name: &str) -> daedyx_tasks_store::test_support::TestDir {
    daedyx_tasks_store::test_support::test_dir(&format!("app-{name}")).unwrap()
}

/// Prüfsumme (FNV-1a, 64 Bit) über den Inhalt; zeigt, ob eine Datei unverändert blieb.
pub fn checksum(path: &std::path::Path) -> u64 {
    std::fs::read(path)
        .unwrap()
        .iter()
        .fold(0xcbf2_9ce4_8422_2325, |h, b| {
            (h ^ u64::from(*b)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}

/// Plan 4.5: Im Datenordner liegen nur `daedyx-tasks.db`, `-wal`, `-shm` und `backups\` mit Sicherungen, seit U2 dazu
/// `updates.json` und `updates\`.
pub fn allowed_files_only(dir: &crate::paths::DataDir) {
    for entry in std::fs::read_dir(dir.path()).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().into_owned();
        let ok = match name.as_str() {
            "daedyx-tasks.db" | "daedyx-tasks.db-wal" | "daedyx-tasks.db-shm" => {
                entry.path().is_file()
            }
            "backups" => entry.path().is_dir(),
            // Update U2: Zustand und Ordner für geladene Setups (`daedyx-updater`).
            "updates.json" => entry.path().is_file(),
            "updates" => entry.path().is_dir(),
            _ => false,
        };
        assert!(ok, "{name}");
    }
    let backups = dir.path().join("backups");
    if backups.is_dir() {
        for entry in std::fs::read_dir(&backups).unwrap() {
            let name = entry.unwrap().file_name().to_string_lossy().into_owned();
            let ok = name.starts_with("daedyx-tasks-v") && name.ends_with(".db");
            assert!(ok, "{name}");
        }
    }
}
