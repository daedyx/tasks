//! Rahmen nach Stil 1: Navigationsleiste (breit 208 px mit Text, ab unter 860 px Fensterbreite schmal 56 px, unter
//! 640 px als Leiste unten), Kopfzeile (48 px, Titel der Seite), Statusleiste, darunter die Seite.
use crate::app::TasksApp;
use crate::page::Page;
use crate::{prefs, status};
use daedyx_ui::a11y::named_scope;
use daedyx_ui::fonts;
use daedyx_ui::theme::look_ui;
use daedyx_ui::tokens::TOKENS;
use daedyx_ui::widgets::{app_tile, nav_entry};
use eframe::egui::{
    Align, FontId, Frame, Layout, Margin, Panel, RichText, Stroke, Ui, accesskit::Role, vec2,
};

/// Unter dieser Breite wird die Leiste schmal, unter `BOTTOM` wandert sie nach unten.
pub const NARROW: f32 = 860.0;
pub const BOTTOM: f32 = 640.0;
const STATUS_HEIGHT: f32 = 48.0;

/// Form der Leiste zur Fensterbreite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rail {
    Wide,
    Narrow,
    Bottom,
}

pub fn rail(width: f32) -> Rail {
    if width < BOTTOM {
        Rail::Bottom
    } else if width < NARROW {
        Rail::Narrow
    } else {
        Rail::Wide
    }
}

fn margin(v: u8) -> Margin {
    Margin::same(i8::try_from(v).unwrap_or(i8::MAX))
}

fn entries(ui: &mut Ui, app: &mut TasksApp, short: bool, pages: &[Page]) {
    for page in pages {
        let label = app.texts.t(page.key());
        if nav_entry(ui, &label, app.active() == *page, short).clicked() {
            app.open(*page);
        }
    }
}

fn side(ui: &mut Ui, app: &mut TasksApp, short: bool) {
    app_tile(ui, &app.texts.t("app.titel"), short);
    ui.add_space(f32::from(TOKENS.spacing.s4));
    let name = app.texts.t("nav.leiste");
    named_scope(ui, "bereiche", Role::TabList, &name, |ui| {
        ui.spacing_mut().item_spacing.y = f32::from(TOKENS.spacing.s1);
        entries(ui, app, short, &Page::MAIN);
        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            entries(ui, app, short, &[Page::Settings]);
        });
    });
}

fn bottom(ui: &mut Ui, app: &mut TasksApp) {
    let name = app.texts.t("nav.leiste");
    named_scope(ui, "bereiche", Role::TabList, &name, |ui| {
        ui.columns(3, |cols| {
            for (col, page) in cols
                .iter_mut()
                .zip([Page::List, Page::Board, Page::Settings])
            {
                entries(col, app, false, &[page]);
            }
        });
    });
}

fn header(ui: &mut Ui, app: &TasksApp) {
    let (name, title) = (app.texts.t("kopf.leiste"), app.texts.t(app.active().key()));
    let look = look_ui(ui);
    named_scope(ui, "kopf", Role::Banner, &name, |ui| {
        ui.horizontal_centered(|ui| {
            let font = FontId::new(TOKENS.font.steps.title.size, fonts::semibold_in(ui.ctx()));
            ui.label(RichText::new(title).font(font).color(look.p.text));
        });
    });
}

pub fn show(ui: &mut Ui, app: &mut TasksApp) {
    let p = look_ui(ui).p;
    let line = Stroke::new(TOKENS.line.fine, p.line);
    let form = rail(ui.ctx().content_rect().width());
    if form == Rail::Bottom {
        Panel::bottom("bereiche_unten")
            .resizable(false)
            .frame(
                Frame::new()
                    .fill(p.sidebar)
                    .stroke(line)
                    .inner_margin(margin(TOKENS.spacing.s2)),
            )
            .show(ui, |ui| bottom(ui, app));
    } else {
        let wide = form == Rail::Wide;
        let size = if wide {
            TOKENS.size.sidebar_wide
        } else {
            TOKENS.size.sidebar_narrow
        };
        Panel::left("bereiche")
            .resizable(false)
            .exact_size(size)
            .frame(
                Frame::new()
                    .fill(p.sidebar)
                    .inner_margin(margin(TOKENS.spacing.s2)),
            )
            .show(ui, |ui| side(ui, app, !wide));
    }
    Panel::bottom("status")
        .resizable(false)
        .exact_size(STATUS_HEIGHT)
        .frame(
            Frame::new()
                .fill(p.sidebar)
                .stroke(line)
                .inner_margin(margin(TOKENS.spacing.s2)),
        )
        .show(ui, |ui| status::show(ui, app));
    Panel::top("kopf")
        .resizable(false)
        .exact_size(TOKENS.size.header)
        .frame(
            Frame::new()
                .fill(p.window)
                .stroke(line)
                .inner_margin(margin(TOKENS.spacing.s4)),
        )
        .show(ui, |ui| header(ui, app));
    ui.allocate_ui(vec2(ui.available_width(), ui.available_height()), |ui| {
        if app.page == Page::Settings {
            prefs::show(ui, app);
        } else {
            daedyx_tasks_ui::show(ui, &mut app.tasks, &app.texts);
        }
    });
}
