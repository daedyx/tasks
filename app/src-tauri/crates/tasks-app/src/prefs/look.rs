//! Darstellung: Sprache und Design (hell/dunkel).
use super::{heading, kind, row};
use crate::app::TasksApp;
use daedyx_ui::i18n::Language;
use daedyx_ui::widgets::button;
use eframe::egui::{Theme, Ui};

pub fn show(ui: &mut Ui, app: &mut TasksApp) {
    heading(ui, &app.texts.t("einstellungen.darstellung"));
    row(ui, &app.texts.t("einstellungen.sprache"), |ui| {
        for lang in Language::ALL {
            if button(ui, kind(app.language == lang), &lang.native_name()).clicked() {
                app.set_language(lang);
            }
        }
    });
    let (light, dark_label) = (app.texts.t("status.hell"), app.texts.t("status.dunkel"));
    row(ui, &app.texts.t("einstellungen.design"), |ui| {
        let dark = ui.visuals().dark_mode;
        if button(ui, kind(!dark), &light).clicked() {
            ui.ctx().set_theme(Theme::Light);
        }
        if button(ui, kind(dark), &dark_label).clicked() {
            ui.ctx().set_theme(Theme::Dark);
        }
    });
}
