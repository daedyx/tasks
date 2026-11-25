//! Feinstellung des Dunkelmodus (T-F3): Helligkeit, Farbton, Tönung, sechs Vorlagen, „Darstellung zurücksetzen“.
//! Die Regler sind in `daedyx-ui` so begrenzt, dass jede Stellung die Kontrastregeln erfüllt.
use super::{heading, hint, kind, row};
use crate::app::TasksApp;
use daedyx_ui::a11y::named_scope;
use daedyx_ui::theme::{DarkTuning, PRESETS};
use daedyx_ui::widgets::{ButtonKind, button};
use eframe::egui::{Slider, Ui, accesskit::Role};

/// Textschlüssel der Vorlage (feste Liste, damit der Wächter jeden Schlüssel sieht).
fn preset_key(id: &str) -> &'static str {
    match id {
        "neutral" => "einstellungen.dunkel.vorlage.neutral",
        "blau" => "einstellungen.dunkel.vorlage.blau",
        "petrol" => "einstellungen.dunkel.vorlage.petrol",
        "gruen" => "einstellungen.dunkel.vorlage.gruen",
        "lila" => "einstellungen.dunkel.vorlage.lila",
        _ => "einstellungen.dunkel.vorlage.warm",
    }
}

/// Ein Regler mit Namen für Screenreader; `true`, wenn er bewegt wurde.
fn slider(ui: &mut Ui, name: &str, value: &mut u16, max: u16) -> bool {
    let mut changed = false;
    row(ui, name, |ui| {
        let r = ui.add(Slider::new(value, 0..=max));
        ui.ctx()
            .accesskit_node_builder(r.id, |n| n.set_label(name.to_owned()));
        changed = r.changed();
    });
    changed
}

pub fn show(ui: &mut Ui, app: &mut TasksApp) {
    let t = app.texts.clone();
    heading(ui, &t.t("einstellungen.dunkel"));
    hint(ui, &t.t("einstellungen.dunkel.hinweis"));
    let now = app.dark_tuning.unwrap_or_default();
    let (mut level, mut hue, mut tint) = (u16::from(now.level), now.hue, u16::from(now.tint));
    let mut changed = slider(ui, &t.t("einstellungen.dunkel.helligkeit"), &mut level, 100);
    changed |= slider(ui, &t.t("einstellungen.dunkel.farbton"), &mut hue, 359);
    changed |= slider(ui, &t.t("einstellungen.dunkel.toenung"), &mut tint, 100);
    let mut next = DarkTuning::new(level.into(), hue.into(), tint.into());
    let name = t.t("einstellungen.dunkel.vorlagen");
    named_scope(ui, "dunkel_vorlagen", Role::Group, &name, |ui| {
        row(ui, &name, |ui| {
            ui.horizontal_wrapped(|ui| {
                for p in &PRESETS {
                    let on = app.dark_tuning.and_then(|d| d.preset()) == Some(p);
                    if button(ui, kind(on), &t.t(preset_key(p.id))).clicked() {
                        next = next.with_preset(p);
                        changed = true;
                    }
                }
            });
        });
    });
    let ctx = ui.ctx().clone();
    if changed {
        app.set_dark_tuning(&ctx, Some(next));
    }
    let reset = t.t("einstellungen.dunkel.zuruecksetzen");
    if button(ui, ButtonKind::Secondary, &reset).clicked() {
        app.set_dark_tuning(&ctx, None);
    }
}
