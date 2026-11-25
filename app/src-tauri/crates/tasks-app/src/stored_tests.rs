//! Einstellungen in `task_meta` (Plan 4.4, Tasks T5): werden gespeichert und beim nächsten Start geladen; kaputte oder
//! unbekannte Werte ergeben den Standardwert; ein gesetzter Umgebungsschalter gewinnt. Wegwerf-Ordner, `MemoryStore`.
use crate::app::TasksApp;
use crate::data::{self, Opened};
use crate::paths::data_dir_in;
use crate::settings::Settings;
use crate::stored::{tuning_of, tuning_text};
use crate::test_support::{allowed_files_only, clock, tmp};
use chrono_tz::Europe::Berlin;
use daedyx_tasks_store::prefs;
use daedyx_tasks_store::secrets::MemoryStore;
use daedyx_ui::i18n::Language;
use daedyx_ui::theme::DarkTuning;
use eframe::egui::{Context, ThemePreference};
use std::path::Path;
use std::sync::Arc;

fn unmeasured(_: &Path) -> Option<u64> {
    None
}

fn start(o: &Opened, settings: &Settings, ctx: &Context) -> TasksApp {
    let mut app = TasksApp::new(settings, Some(data::store(o, Berlin, clock())));
    app.load_prefs(ctx, Arc::clone(&o.db), settings);
    app
}

#[test]
fn einstellungen_werden_gespeichert_und_beim_naechsten_start_geladen() {
    let dir = data_dir_in(&tmp("t5-prefs")).unwrap();
    let vault = MemoryStore::default();
    let o = data::open(&dir, &vault, &unmeasured, Berlin, clock()).unwrap();
    let ctx = Context::default();
    let mut app = start(&o, &Settings::default(), &ctx);
    let list = app.tasks.lists[1].id.clone();
    app.set_language(Language::Fr);
    ctx.set_theme(ThemePreference::Dark);
    app.bg.window.set_keep_running(false);
    app.bg.set_show_titles(true);
    let tuning = DarkTuning {
        level: 40,
        hue: 185,
        tint: 40,
    };
    app.set_dark_tuning(&ctx, Some(tuning));
    app.tasks.board_lists.insert(list.clone());
    app.save_prefs(&ctx);
    drop(app);
    let ctx2 = Context::default();
    let again = start(&o, &Settings::default(), &ctx2);
    assert_eq!(again.language, Language::Fr);
    assert_eq!(ctx2.options(|o| o.theme_preference), ThemePreference::Dark);
    assert!(!again.bg.window.keep_running());
    assert!(again.bg.show_titles());
    assert_eq!(again.dark_tuning, Some(tuning));
    assert!(again.tasks.board_lists.contains(&list));
    drop((again, o));
    allowed_files_only(&dir);
}

#[test]
fn zurueck_zum_standard_entfernt_den_eintrag() {
    let dir = data_dir_in(&tmp("t5-prefs-weg")).unwrap();
    let o = data::open(&dir, &MemoryStore::default(), &unmeasured, Berlin, clock()).unwrap();
    let ctx = Context::default();
    let mut app = start(&o, &Settings::default(), &ctx);
    app.set_dark_tuning(
        &ctx,
        Some(DarkTuning {
            level: 10,
            hue: 0,
            tint: 0,
        }),
    );
    let list = app.tasks.lists[0].id.clone();
    app.tasks.board_lists.insert(list.clone());
    app.save_prefs(&ctx);
    app.set_dark_tuning(&ctx, None);
    app.tasks.board_lists.remove(&list);
    app.save_prefs(&ctx);
    let conn = o.db.lock().unwrap();
    assert_eq!(prefs::get(&conn, "app.dark_tuning"), None);
    assert!(prefs::board_modes(&conn).is_empty());
}

#[test]
fn kaputte_werte_ergeben_den_standard_und_umgebung_gewinnt() {
    let dir = data_dir_in(&tmp("t5-prefs-kaputt")).unwrap();
    let o = data::open(&dir, &MemoryStore::default(), &unmeasured, Berlin, clock()).unwrap();
    {
        let conn = o.db.lock().unwrap();
        for (k, v) in [
            ("app.language", "klingonisch"),
            ("app.theme", "lila"),
            ("app.keep_running", "ja"),
            ("app.dark_tuning", "999,999,999"),
            ("app.board_mode.fehlt", "board"),
        ] {
            prefs::set(&conn, k, v).unwrap();
        }
    }
    let ctx = Context::default();
    let app = start(&o, &Settings::default(), &ctx);
    assert_eq!(app.language, Language::De);
    assert_eq!(ctx.options(|o| o.theme_preference), ThemePreference::System);
    assert!(app.bg.window.keep_running(), "Standard an (T-F9)");
    assert_eq!(app.dark_tuning, None);
    assert!(app.tasks.board_lists.is_empty(), "Liste gibt es nicht");
    assert!(
        prefs::board_modes(&o.db.lock().unwrap()).is_empty(),
        "Eintrag der fehlenden Liste entfernt (H5-3)"
    );
    {
        let conn = o.db.lock().unwrap();
        prefs::set(&conn, "app.language", "en").unwrap();
        prefs::set(&conn, "app.theme", "light").unwrap();
    }
    let env = Settings {
        language: Language::Fr,
        language_from_env: true,
        dark: Some(true),
        ..Settings::default()
    };
    let ctx2 = Context::default();
    let app2 = start(&o, &env, &ctx2);
    assert_eq!(app2.language, Language::Fr, "DAEDYX_LANG gewinnt");
    assert_eq!(
        ctx2.options(|o| o.theme_preference),
        ThemePreference::System,
        "DAEDYX_THEME gewinnt (setzt das Bild)"
    );
}

#[test]
fn feinstellung_als_text_nur_in_den_reglergrenzen() {
    let t = DarkTuning {
        level: 100,
        hue: 359,
        tint: 100,
    };
    assert_eq!(tuning_of(&tuning_text(t)), Some(t));
    for bad in [
        "", "1,2", "1,2,3,4", "101,0,0", "0,360,0", "0,0,101", "-1,0,0", "a,b,c", "1, 2 ,3x",
    ] {
        assert_eq!(tuning_of(bad), None, "{bad:?}");
    }
    assert_eq!(
        tuning_of(" 5, 6 ,7"),
        Some(DarkTuning {
            level: 5,
            hue: 6,
            tint: 7
        })
    );
}
