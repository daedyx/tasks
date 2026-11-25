//! Einstellungen: Darstellung (mit Feinstellung des Dunkelmodus), Start und Hintergrund, Über (Version und
//! Lizenzen; Updates kommen mit der gemeinsamen Update-Bibliothek U1, nicht in T4). Schmale Spalte (höchstens 620 px),
//! Zeilen „Titel links, Bedienelement rechts“ (Stil 1).
mod about;
mod background;
mod dark;
pub(crate) mod license_texts;
pub(crate) mod licenses;
mod look;

use crate::app::TasksApp;
use daedyx_ui::a11y::{mark_heading, named_scope};
use daedyx_ui::fonts;
use daedyx_ui::theme::look_ui;
use daedyx_ui::tokens::TOKENS;
use daedyx_ui::widgets::ButtonKind;
use eframe::egui::{FontId, Frame, Margin, RichText, ScrollArea, Ui, accesskit::Role};

const COLUMN: f32 = 620.0;

pub(crate) fn kind(on: bool) -> ButtonKind {
    if on {
        ButtonKind::Primary
    } else {
        ButtonKind::Secondary
    }
}

pub(crate) fn heading(ui: &mut Ui, text: &str) {
    let look = look_ui(ui);
    let font = FontId::new(
        TOKENS.font.steps.emphasis.size,
        fonts::semibold_in(ui.ctx()),
    );
    let resp = ui.label(RichText::new(text).font(font).color(look.p.text));
    mark_heading(ui, resp.id, 2, text);
}

pub(crate) fn row(ui: &mut Ui, title: &str, add: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.label(title);
        ui.add_space(f32::from(TOKENS.spacing.s4));
        add(ui);
    });
    ui.add_space(f32::from(TOKENS.spacing.s3));
}

/// Kleiner Hinweis unter einem Bedienelement.
pub(crate) fn hint(ui: &mut Ui, text: &str) {
    let look = look_ui(ui);
    ui.label(RichText::new(text).small().color(look.p.text_muted));
    ui.add_space(f32::from(TOKENS.spacing.s3));
}

pub fn show(ui: &mut Ui, app: &mut TasksApp) {
    let p = look_ui(ui).p;
    let inner = Margin::same(i8::try_from(TOKENS.spacing.s6).unwrap_or(i8::MAX));
    let gap = f32::from(TOKENS.spacing.s6);
    Frame::new()
        .fill(p.surface)
        .inner_margin(inner)
        .show(ui, |ui| {
            ui.set_min_size(ui.available_size());
            let name = app.texts.t("nav.einstellungen");
            named_scope(ui, "einstellungen", Role::Group, &name, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.set_max_width(COLUMN);
                    look::show(ui, app);
                    ui.add_space(gap);
                    dark::show(ui, app);
                    ui.add_space(gap);
                    background::show(ui, app);
                    ui.add_space(gap);
                    about::show(ui, app);
                });
            });
        });
}
