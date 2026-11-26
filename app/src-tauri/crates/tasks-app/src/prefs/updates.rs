//! Updates (U2): Lage, Angebot mit Notiz als reinem Text, „Jetzt aktualisieren“, „Nach Updates suchen“, Schalter
//! „Beim Start suchen“. Das Bauteil kommt aus `daedyx-updater` (gleich in Desk); hier nur Überschrift und Platz.
use super::heading;
use crate::app::TasksApp;
use eframe::egui::Ui;

pub fn show(ui: &mut Ui, app: &mut TasksApp) {
    let t = app.texts.clone();
    heading(ui, &t.t("update.titel"));
    daedyx_updater::panel::show(ui, &mut app.updater, &t);
}
