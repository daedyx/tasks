//! Einstellungen der App dauerhaft in `task_meta` (Plan 4.4, Tasks T5) über die Positivliste des Stores
//! (`daedyx_tasks_store::prefs`): Design, Sprache, „Im Infobereich weiterlaufen“, Titel in Benachrichtigungen,
//! Feinstellung des Dunkelmodus und der Board-Modus je Liste. Beim Start gelesen, danach in jedem Bild mit dem letzten
//! Stand verglichen und nur Geändertes geschrieben. Unbekannte oder kaputte Werte ergeben den Standardwert. Ein
//! gesetzter Umgebungsschalter (`DAEDYX_LANG`, `DAEDYX_THEME`) gewinnt beim Start. Autostart bleibt in der Registry.
use crate::app::TasksApp;
use crate::reminders::SharedConn;
use crate::settings::Settings;
use daedyx_tasks_store::prefs;
use daedyx_ui::i18n::Language;
use daedyx_ui::theme::DarkTuning;
use eframe::egui::{Context, ThemePreference};
use std::collections::BTreeSet;

/// Was gespeichert wird, als Text je Schlüssel.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub values: Vec<(&'static str, Option<String>)>,
    pub board: BTreeSet<String>,
}

pub struct Stored {
    db: SharedConn,
    last: Snapshot,
}

pub fn theme_text(t: ThemePreference) -> &'static str {
    match t {
        ThemePreference::Dark => "dark",
        ThemePreference::Light => "light",
        ThemePreference::System => "system",
    }
}

pub fn theme_of(text: &str) -> Option<ThemePreference> {
    Some(match text {
        "dark" => ThemePreference::Dark,
        "light" => ThemePreference::Light,
        "system" => ThemePreference::System,
        _ => return None,
    })
}

pub fn language_text(l: Language) -> &'static str {
    match l {
        Language::De => "de",
        Language::En => "en",
        Language::Fr => "fr",
    }
}

pub fn language_of(text: &str) -> Option<Language> {
    Language::ALL
        .into_iter()
        .find(|l| language_text(*l) == text)
}

pub fn flag_of(text: &str) -> Option<bool> {
    match text {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}

pub fn tuning_text(t: DarkTuning) -> String {
    format!("{},{},{}", t.level, t.hue, t.tint)
}

/// „Helligkeit,Farbton,Tönung“ in den Grenzen der Regler (0–100, 0–359, 0–100); alles andere ergibt `None`.
pub fn tuning_of(text: &str) -> Option<DarkTuning> {
    let mut parts = text.split(',').map(|p| p.trim().parse::<u16>().ok());
    let (level, hue, tint) = (parts.next()??, parts.next()??, parts.next()??);
    if parts.next().is_some() || level > 100 || hue > 359 || tint > 100 {
        return None;
    }
    Some(DarkTuning {
        level: u8::try_from(level).ok()?,
        hue,
        tint: u8::try_from(tint).ok()?,
    })
}

fn snapshot(app: &TasksApp, ctx: &Context) -> Snapshot {
    let flag = |b: bool| Some(if b { "1" } else { "0" }.to_owned());
    Snapshot {
        values: vec![
            (
                prefs::THEME,
                Some(theme_text(ctx.options(|o| o.theme_preference)).to_owned()),
            ),
            (
                prefs::LANGUAGE,
                Some(language_text(app.language).to_owned()),
            ),
            (prefs::KEEP_RUNNING, flag(app.bg.window.keep_running())),
            (prefs::NOTIFY_TITLES, flag(app.bg.show_titles())),
            (prefs::DARK_TUNING, app.dark_tuning.map(tuning_text)),
        ],
        board: app.tasks.board_lists.iter().cloned().collect(),
    }
}

fn write(db: &SharedConn, last: &Snapshot, now: &Snapshot) {
    let Ok(conn) = db.lock() else {
        return;
    };
    for ((key, old), (_, new)) in last.values.iter().zip(&now.values) {
        if old != new {
            let _ = match new {
                Some(v) => prefs::set(&conn, key, v),
                None => prefs::remove(&conn, key),
            };
        }
    }
    for id in now.board.difference(&last.board) {
        let _ = prefs::set(&conn, &format!("{}{id}", prefs::BOARD_MODE), "board");
    }
    for id in last.board.difference(&now.board) {
        let _ = prefs::remove(&conn, &format!("{}{id}", prefs::BOARD_MODE));
    }
}

impl TasksApp {
    /// Gespeicherte Einstellungen übernehmen (einmal beim Start) und ab jetzt speichern.
    pub fn load_prefs(&mut self, ctx: &Context, db: SharedConn, settings: &Settings) {
        if let Ok(conn) = db.lock() {
            let get = |k: &str| prefs::get(&conn, k);
            if !settings.language_from_env
                && let Some(l) = get(prefs::LANGUAGE).as_deref().and_then(language_of)
            {
                self.set_language(l);
            }
            if settings.dark.is_none()
                && let Some(t) = get(prefs::THEME).as_deref().and_then(theme_of)
            {
                ctx.set_theme(t);
            }
            if let Some(on) = get(prefs::KEEP_RUNNING).as_deref().and_then(flag_of) {
                self.bg.window.set_keep_running(on);
            }
            if let Some(on) = get(prefs::NOTIFY_TITLES).as_deref().and_then(flag_of) {
                self.bg.set_show_titles(on);
            }
            if let Some(t) = get(prefs::DARK_TUNING).as_deref().and_then(tuning_of) {
                self.set_dark_tuning(ctx, Some(t));
            }
            let ids: BTreeSet<&str> = self.tasks.lists.iter().map(|l| l.id.as_str()).collect();
            let (keep, stale): (Vec<_>, Vec<_>) = prefs::board_modes(&conn)
                .into_iter()
                .partition(|(id, v)| v == "board" && ids.contains(id.as_str()));
            // Einträge gelöschter Listen (oder mit unbekanntem Wert) gleich entfernen (Review T5, H5-3).
            for (id, _) in stale {
                let _ = prefs::remove(&conn, &format!("{}{id}", prefs::BOARD_MODE));
            }
            self.tasks.board_lists = keep.into_iter().map(|(id, _)| id).collect();
        }
        let last = snapshot(self, ctx);
        self.stored = Some(Stored { db, last });
    }

    /// Geänderte Einstellungen schreiben (in jedem Bild aufgerufen; ohne Änderung keine Schreibvorgänge).
    pub fn save_prefs(&mut self, ctx: &Context) {
        let now = snapshot(self, ctx);
        if let Some(stored) = &mut self.stored
            && stored.last != now
        {
            write(&stored.db, &stored.last, &now);
            stored.last = now;
        }
    }
}
