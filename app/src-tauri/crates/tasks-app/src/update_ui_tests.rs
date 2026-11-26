//! Update in der Oberfläche von Tasks (U2, kittest): Abschnitt „Updates“ in den Einstellungen, Hinweis in der
//! Statuszeile nur bei einem Anlass, Klick darauf öffnet die Einstellungen. Ohne Ablauf (`Updater::disabled`).
use crate::page::Page;
use crate::settings::Settings;
use crate::t4_ui_tests::settings_page;
use crate::test_support::{app, harness};
use daedyx_updater::Status;
use eframe::egui::accesskit::{Role, Toggled};
use egui_kittest::kittest::{NodeT, Queryable};

#[test]
fn einstellungen_haben_den_abschnitt_updates() {
    let mut h = settings_page();
    let _ = h.get_by_label("Updates");
    let _ = h.get_by_role_and_label(Role::Button, "Nach Updates suchen");
    let auto = "Beim Start nach Updates suchen";
    let toggled = |h: &egui_kittest::Harness<'static, crate::app::TasksApp>| {
        h.get_by_role_and_label(Role::CheckBox, auto)
            .accesskit_node()
            .toggled()
    };
    assert_eq!(toggled(&h), Some(Toggled::True), "Voreinstellung an (U-F2)");
    h.get_by_role_and_label(Role::CheckBox, auto).click();
    h.run();
    assert_eq!(toggled(&h), Some(Toggled::False));
    assert!(!h.state().updater.auto_check());
    let _ = h.get_by_label("Höchstens einmal am Tag. Installiert wird nur nach deinem Klick.");
}

#[test]
fn statuszeile_zeigt_ein_angebot_und_fuehrt_zu_den_einstellungen() {
    let mut a = app(&Settings::default());
    a.updater.set_status(Status::Available {
        version: "9.9.9".into(),
        notes: "Erfundene Neuerung.".into(),
        important: false,
    });
    let mut h = harness(a, 1280.0);
    let status = h.get_by_role(Role::Status);
    status.get_by_label("Update 9.9.9 verfügbar").click();
    h.run();
    h.run();
    assert_eq!(h.state().page, Page::Settings);
    let _ = h.get_by_label("Version 9.9.9 ist verfügbar.");
}

#[test]
fn ohne_anlass_bleibt_die_statuszeile_still() {
    let mut a = app(&Settings::default());
    a.updater.set_status(Status::UpToDate);
    let h = harness(a, 1280.0);
    let status = h.get_by_role(Role::Status);
    assert!(status.query_by_label_contains("Update").is_none());
}
