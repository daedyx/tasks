//! Einstellungen (T2: Darstellung und Über). Schmale Spalte (höchstens 620 px), Zeilen „Titel links, Bedienelement
//! rechts“ (Stil 1). Weitere Seiten (Start und Hintergrund, Updates) kommen in T4.
use crate::app::TasksApp;
use daedyx_ui::a11y::{mark_heading, named_scope};
use daedyx_ui::fonts;
use daedyx_ui::i18n::Language;
use daedyx_ui::theme::look_ui;
use daedyx_ui::tokens::TOKENS;
use daedyx_ui::widgets::{ButtonKind, button};
use eframe::egui::{FontId, Frame, Margin, RichText, ScrollArea, Theme, Ui, accesskit::Role};

const COLUMN: f32 = 620.0;

fn kind(on: bool) -> ButtonKind {
    if on {
        ButtonKind::Primary
    } else {
        ButtonKind::Secondary
    }
}

fn heading(ui: &mut Ui, text: &str) {
    let look = look_ui(ui);
    let font = FontId::new(
        TOKENS.font.steps.emphasis.size,
        fonts::semibold_in(ui.ctx()),
    );
    let resp = ui.label(RichText::new(text).font(font).color(look.p.text));
    mark_heading(ui, resp.id, 2, text);
}

fn row(ui: &mut Ui, title: &str, add: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.label(title);
        ui.add_space(f32::from(TOKENS.spacing.s4));
        add(ui);
    });
    ui.add_space(f32::from(TOKENS.spacing.s3));
}

fn appearance(ui: &mut Ui, app: &mut TasksApp) {
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

fn about(ui: &mut Ui, app: &TasksApp) {
    let look = look_ui(ui);
    heading(ui, &app.texts.t("einstellungen.ueber"));
    let version = app.texts.tf(
        "einstellungen.version",
        &[("version", env!("CARGO_PKG_VERSION"))],
    );
    ui.label(version);
    ui.label(RichText::new(app.texts.t("einstellungen.hinweis")).color(look.p.text_muted));
}

pub fn show(ui: &mut Ui, app: &mut TasksApp) {
    let p = look_ui(ui).p;
    let inner = Margin::same(i8::try_from(TOKENS.spacing.s6).unwrap_or(i8::MAX));
    Frame::new()
        .fill(p.surface)
        .inner_margin(inner)
        .show(ui, |ui| {
            ui.set_min_size(ui.available_size());
            let name = app.texts.t("nav.einstellungen");
            named_scope(ui, "einstellungen", Role::Group, &name, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.set_max_width(COLUMN);
                    appearance(ui, app);
                    ui.add_space(f32::from(TOKENS.spacing.s6));
                    about(ui, app);
                });
            });
        });
}
