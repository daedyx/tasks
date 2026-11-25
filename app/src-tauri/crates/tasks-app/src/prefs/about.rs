//! Über: Version und Lizenzen. Updates kommen mit der gemeinsamen Update-Bibliothek (U1), nicht in T4.
use super::{heading, hint};
use crate::app::TasksApp;
use eframe::egui::Ui;

pub fn show(ui: &mut Ui, app: &TasksApp) {
    let t = &app.texts;
    heading(ui, &t.t("einstellungen.ueber"));
    ui.label(t.tf(
        "einstellungen.version",
        &[("version", env!("CARGO_PKG_VERSION"))],
    ));
    hint(ui, &t.t("einstellungen.hinweis"));
    heading(ui, &t.t("einstellungen.lizenzen"));
    ui.label(t.t("einstellungen.lizenz"));
    hint(ui, &t.t("einstellungen.lizenzen.bausteine"));
}
