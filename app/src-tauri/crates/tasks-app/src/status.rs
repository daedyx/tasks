//! Statusleiste: Hinweis „Vorschau mit erfundenen Daten“, Hinweis auf ein Update (U2, nur wenn es etwas zu sagen gibt;
//! Klick öffnet die Einstellungen mit dem Abschnitt „Updates“) und der Umschalter Hell/Dunkel (wie Desk).
use crate::app::TasksApp;
use crate::page::Page;
use daedyx_ui::a11y::named_scope;
use daedyx_ui::widgets::{BadgeTone, ButtonKind, badge, button};
use eframe::egui::{Align, Layout, Theme, Ui, accesskit::Role};

fn kind(on: bool) -> ButtonKind {
    if on {
        ButtonKind::Primary
    } else {
        ButtonKind::Quiet
    }
}

pub fn show(ui: &mut Ui, app: &mut TasksApp) {
    let t = app.texts.clone();
    let (name, preview) = (t.t("status.leiste"), t.t("status.hinweis"));
    let (light, dark_label) = (t.t("status.hell"), t.t("status.dunkel"));
    let mut open_updates = false;
    named_scope(ui, "status", Role::Status, &name, |ui| {
        ui.horizontal_centered(|ui| {
            let _ = badge(ui, BadgeTone::Warning, &preview);
            if let Some(r) = daedyx_updater::panel::status_badge(ui, &app.updater, &t) {
                open_updates = r.clicked();
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let dark = ui.visuals().dark_mode;
                if button(ui, kind(dark), &dark_label).clicked() {
                    ui.ctx().set_theme(Theme::Dark);
                }
                if button(ui, kind(!dark), &light).clicked() {
                    ui.ctx().set_theme(Theme::Light);
                }
            });
        });
    });
    if open_updates {
        app.open(Page::Settings);
    }
}
