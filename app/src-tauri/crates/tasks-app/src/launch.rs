//! Start des Programms: Startprobe, Startfehler-Fenster oder die App mit erfundenen Daten.
use crate::app::TasksApp;
use crate::ids::{self, APP_ID};
use crate::settings::Settings;
use crate::{seed, startfehler};
use daedyx_tasks_ui::Clock;
use daedyx_ui::screenshot::Shot;
use daedyx_ui::theme;
use daedyx_ui::tokens::AppAccent;
use eframe::egui;
use std::process::ExitCode;

/// Rückgabe-Codes ohne Text.
pub mod code {
    pub const GUI: u8 = 6;
    pub const STARTUP_ERROR: u8 = 9;
}

/// `--start-probe`: Programm laden und sofort mit 0 enden (belegt, dass Windows es laden kann, Bericht M7b). Gehören
/// Kennung und Tresor nicht zur selben Fassung, endet sie mit 1.
pub fn start_probe(args: &[String]) -> bool {
    args.iter().any(|a| a == "--start-probe")
}

pub fn run(settings: Settings, args: &[String]) -> ExitCode {
    if start_probe(args) {
        return if ids::consistent() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    let texts = crate::texts::texts(settings.language);
    if let Some(code) = settings.startup_error {
        let window = startfehler::window(code, &texts);
        let _ = daedyx_ui::startup::run(APP_ID, &texts.t("app.titel"), AppAccent::Tasks, window);
        return ExitCode::from(code::STARTUP_ERROR);
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([480.0, 480.0])
            .with_title(texts.t("app.titel")),
        // Spike S1: wgpu hält im Leerlauf einen Kern auf 100 %, glow nicht.
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    let result = eframe::run_native(
        APP_ID,
        options,
        Box::new(move |cc| {
            theme::install(&cc.egui_ctx, AppAccent::Tasks);
            let store = seed::demo(seed::zone(), Clock::System);
            Ok(Box::new(
                TasksApp::new(&settings, store).with_shot(Shot::from_env()),
            ))
        }),
    );
    if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(code::GUI)
    }
}
