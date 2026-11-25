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
