//! Start und Hintergrund: Mit Windows starten (versteckt), Hinweis bei Sperre im Task-Manager, „Im Infobereich
//! weiterlaufen“ (T-F9), Aufgabentitel in Benachrichtigungen (T-F6, Standard aus), Testbenachrichtigung mit
//! erfundenem Inhalt.
use super::{heading, hint};
use crate::app::TasksApp;
use daedyx_background::autostart::State;
use daedyx_ui::theme::look_ui;
use daedyx_ui::tokens::TOKENS;
use daedyx_ui::widgets::{ButtonKind, button};
use eframe::egui::{RichText, Ui};

pub fn show(ui: &mut Ui, app: &mut TasksApp) {
    let t = app.texts.clone();
    let look = look_ui(ui);
    heading(ui, &t.t("einstellungen.hintergrund"));

    let mut on = matches!(app.bg.autostart_state, Some(State::On | State::Blocked));
    if ui.checkbox(&mut on, t.t("hintergrund.autostart")).changed() {
        app.bg.set_autostart(on);
    }
    let state_key = match app.bg.autostart_state {
        Some(State::Blocked) => Some("hintergrund.autostart.gesperrt"),
        None => Some("hintergrund.autostart.unbekannt"),
        _ => None,
    };
    if let Some(k) = state_key {
        ui.label(RichText::new(t.t(k)).color(look.p.state.warning.text));
    }
    if app.bg.autostart_error {
        ui.label(RichText::new(t.t("hintergrund.autostart.fehler")).color(look.p.state.error.text));
    }
    ui.add_space(f32::from(TOKENS.spacing.s3));

    let mut keep = app.bg.window.keep_running();
    if ui
        .checkbox(&mut keep, t.t("hintergrund.weiterlaufen"))
        .changed()
    {
        app.set_keep_running(keep);
    }
    hint(ui, &t.t("hintergrund.weiterlaufen.hinweis"));

    // Titel stehen sonst unverschlüsselt im Info-Center und auf dem Sperrbildschirm (Desk N-1, Tasks T-F6).
    let mut titles = app.bg.show_titles();
    if ui
        .checkbox(&mut titles, t.t("hintergrund.titel_zeigen"))
        .changed()
    {
        app.bg.set_show_titles(titles);
    }
    hint(ui, &t.t("hintergrund.titel_zeigen.hinweis"));

    if button(ui, ButtonKind::Secondary, &t.t("hintergrund.test")).clicked() {
        app.bg.send_test(
            &t.t("hintergrund.test.titel"),
            &t.t("hintergrund.test.text"),
        );
    }
    match app.bg.test_sent {
        Some(true) => {
            ui.label(RichText::new(t.t("hintergrund.test.ok")).color(look.p.state.ok.text));
        }
        Some(false) => {
            ui.label(RichText::new(t.t("hintergrund.test.fehler")).color(look.p.state.error.text));
        }
        None => {}
    }
}
