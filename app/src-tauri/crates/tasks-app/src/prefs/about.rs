//! Über: Version, eigene Lizenz und die Liste aller mitgebauten Bibliotheken mit Filter; ein gewählter Eintrag zeigt
//! Urheber und vollständigen Lizenztext (wie Desk, Tasks T6). Dieselben Texte liefert der Installer als `Lizenzen.txt`.
//! Updates stehen im eigenen Abschnitt darüber (`updates.rs`, U2). Filter und Auswahl liegen nur im Speicher der
//! Oberfläche.
use super::license_texts::{TEXTS, full_text};
use super::licenses::{licenses, matches};
use super::{heading, hint};
use crate::app::TasksApp;
use daedyx_ui::a11y::mark_heading;
use daedyx_ui::theme::look_ui;
use daedyx_ui::widgets::text_input;
use eframe::egui::{RichText, ScrollArea, Ui};

const ROW: f32 = 22.0;
const LIST_H: f32 = 260.0;
const TEXT_H: f32 = 320.0;

/// Filter und gewählter Eintrag (Name, Version) der Lizenzliste.
#[derive(Clone, Default)]
struct View {
    filter: String,
    open: Option<(String, String)>,
}

pub fn show(ui: &mut Ui, app: &TasksApp) {
    let t = &app.texts;
    let look = look_ui(ui);
    heading(ui, &t.t("einstellungen.ueber"));
    ui.label(t.tf(
        "einstellungen.version",
        &[("version", env!("CARGO_PKG_VERSION"))],
    ));
    hint(ui, &t.t("einstellungen.hinweis"));
    heading(ui, &t.t("einstellungen.lizenzen"));
    ui.label(t.t("einstellungen.lizenz"));

    let id = ui.make_persistent_id("tasks_lizenzliste");
    let mut view: View = ui.data_mut(|d| d.get_temp(id)).unwrap_or_default();
    let all = licenses();
    let count = all.len().to_string();
    heading(
        ui,
        &t.tf("einstellungen.ueber.bibliotheken", &[("anzahl", &count)]),
    );
    let _ = text_input(
        ui,
        &mut view.filter,
        &t.t("einstellungen.ueber.filter"),
        &t.t("einstellungen.ueber.filter.hinweis"),
        None,
    );
    let shown: Vec<_> = all.iter().filter(|l| matches(l, &view.filter)).collect();
    let mut clicked = None;
    ScrollArea::vertical()
        .id_salt("lizenzliste")
        .max_height(LIST_H)
        .auto_shrink([false, true])
        .show_rows(ui, ROW, shown.len(), |ui, range| {
            for l in shown.get(range).unwrap_or_default() {
                ui.horizontal(|ui| {
                    let key = (l.name.to_owned(), l.version.to_owned());
                    let on = view.open.as_ref() == Some(&key);
                    if ui
                        .selectable_label(on, format!("{} {}", l.name, l.version))
                        .clicked()
                    {
                        clicked = Some(key);
                    }
                    ui.label(RichText::new(l.license).small().color(look.p.text_muted));
                });
            }
        });
    if let Some(key) = clicked {
        view.open = (view.open.as_ref() != Some(&key)).then_some(key);
    }
    // Vollständiger Hinweis (Urheber und Lizenztext) des gewählten Eintrags.
    match view.open.as_ref() {
        None => hint(ui, &t.t("einstellungen.ueber.lizenz.waehlen")),
        Some((name, version)) => {
            let title = format!("{name} {version}");
            let resp = ui.label(RichText::new(&title).strong());
            mark_heading(ui, resp.id, 3, &title);
            let body = full_text(name, version, TEXTS).unwrap_or_default();
            ScrollArea::vertical()
                .id_salt("lizenztext")
                .max_height(TEXT_H)
                .show(ui, |ui| {
                    ui.label(RichText::new(body).monospace().small());
                });
        }
    }
    ui.data_mut(|d| d.insert_temp(id, view));
}
