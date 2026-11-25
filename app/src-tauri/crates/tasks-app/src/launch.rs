//! Start des Programms: Startprobe, Startfehler-Fenster oder die App mit erfundenen Daten und Hintergrund (T4):
//! Einzelinstanz (zweiter Start holt das Fenster, `--quit-running` beendet die laufende Vorschau), App-Kennung für
//! Benachrichtigungen, Fenster (versteckt mit `--hidden`), Symbol im Infobereich, Takt der Erinnerungen, Autostart.
//! Alles mit den Kennungen aus `ids.rs`.
use crate::app::TasksApp;
use crate::ids::{self, APP_ID, BACKGROUND};
use crate::settings::Settings;
use crate::{seed, startfehler};
use daedyx_background::args::Args;
use daedyx_background::aumid;
use daedyx_background::instance::{self, Role};
use daedyx_background::notify::WinToast;
use daedyx_background::start::{self as bg, code as bg_code};
use daedyx_background::tray::{self, Tray, TrayTexts};
use daedyx_tasks_ui::Clock;
use daedyx_ui::screenshot::Shot;
use daedyx_ui::theme;
use daedyx_ui::tokens::AppAccent;
use eframe::egui;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

/// Rückgabe-Codes ohne Text (2–8 kommen aus `daedyx_background::start::code`).
pub mod code {
    pub const GUI: u8 = 6;
    pub const STARTUP_ERROR: u8 = 9;
}

/// Hält das Symbol im Infobereich am Leben.
struct Shell {
    app: TasksApp,
    _tray: Option<Tray>,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.app.logic(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.app.frame(ui);
    }
}

pub fn run(settings: Settings, args: Args) -> ExitCode {
    // `--start-probe`: laden und sofort enden (Bericht M7b). Gehören die Kennungen nicht zur selben Fassung: 1.
    if args.start_probe {
        return if ids::consistent() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    let texts = crate::texts::texts(settings.language);
    if let Some(code) = settings.startup_error {
        let window = startfehler::window(code, &texts);
        let _ = daedyx_ui::startup::run(
            APP_ID,
            &texts.t("app.titel"),
            AppAccent::Tasks,
            settings.theme(),
            window,
        );
        return ExitCode::from(code::STARTUP_ERROR);
    }
    if args.cleanup {
        return bg::cleanup(&BACKGROUND);
    }
    if args.probe_toast {
        return bg::probe_toast(
            &BACKGROUND,
            &texts.t("app.titel"),
            &texts.t("hintergrund.test.titel"),
            &texts.t("hintergrund.test.text"),
        );
    }
    let Ok(rt) = bg::runtime() else {
        return ExitCode::from(bg_code::RUNTIME);
    };
    let pipe = bg::pipe(&BACKGROUND);
    let listener = match rt.block_on(async { instance::acquire(&pipe) }) {
        Ok(Role::Primary(_)) if args.quit_running => return ExitCode::SUCCESS,
        Ok(Role::Primary(l)) => l,
        Ok(Role::Secondary) => {
            let word = if args.quit_running {
                instance::QUIT
            } else {
                instance::SHOW
            };
            return match rt.block_on(instance::notify_primary(&pipe, word)) {
                Ok(()) => ExitCode::SUCCESS,
                Err(_) => ExitCode::from(bg_code::SECOND_INSTANCE_FAILED),
            };
        }
        Err(_) => return ExitCode::from(bg_code::PIPE),
    };
    let _ = aumid::register(APP_ID, &texts.t("app.titel"));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([480.0, 480.0])
            .with_title(texts.t("app.titel"))
            .with_visible(!args.hidden),
        // Spike S1: wgpu hält im Leerlauf einen Kern auf 100 %, glow nicht.
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    let handle = rt.handle().clone();
    let result = eframe::run_native(
        APP_ID,
        options,
        Box::new(move |cc| {
            theme::install(&cc.egui_ctx, AppAccent::Tasks);
            let zone = seed::zone();
            let (store, db) = seed::demo_shared(zone, Clock::System).unzip();
            let mut app = TasksApp::new(&settings, store).with_shot(Shot::from_env());
            let sink = app.bg.sink.clone();
            sink.attach(&cc.egui_ctx);
            app.bg.notifier = Arc::new(WinToast {
                aumid: APP_ID,
                on_click: Some(sink.clone()),
            });
            app.bg.set_autostart_store(bg::autostart_store(&BACKGROUND));
            if args.hidden {
                app.bg.window.start_hidden();
            }
            if let Some(db) = db {
                app.start_reminders(Duration::from_secs(args.takt_secs), db, zone.name());
            }
            let tray_texts = TrayTexts {
                tooltip: texts.t("app.titel"),
                open: texts.t("hintergrund.symbol.oeffnen"),
                quit: texts.t("hintergrund.symbol.beenden"),
            };
            let dark = cc.egui_ctx.global_style().visuals.dark_mode;
            let rgba = theme::look(&cc.egui_ctx, dark).accent.full.to_array();
            let tray = tray::create(&sink, &tray_texts, rgba).ok();
            handle.spawn(listener.serve(sink));
            Ok(Box::new(Shell { app, _tray: tray }))
        }),
    );
    rt.shutdown_timeout(Duration::from_secs(1));
    if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(code::GUI)
    }
}
