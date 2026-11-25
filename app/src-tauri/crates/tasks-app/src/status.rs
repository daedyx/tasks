//! Statusleiste: Hinweis „Vorschau mit erfundenen Daten“ und der Umschalter Hell/Dunkel (wie Desk).
use crate::app::TasksApp;
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
    let t = &app.texts;
    let (name, preview) = (t.t("status.leiste"), t.t("status.vorschau"));
    let (light, dark_label) = (t.t("status.hell"), t.t("status.dunkel"));
    named_scope(ui, "status", Role::Status, &name, |ui| {
        ui.horizontal_centered(|ui| {
            let _ = badge(ui, BadgeTone::Warning, &preview);
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
}
