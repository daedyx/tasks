//! Hintergrund und Einstellungen in der App (T4): Schalter „Im Infobereich weiterlaufen“ (aus = Schließen beendet,
//! Entwurf wird vorher übernommen), Benachrichtigungen ohne Titel als Standard, Autostart, Testbenachrichtigung,
//! Feinstellung des Dunkelmodus. Nur erfundene Daten, Autostart nur als Ersatz im Speicher.
use crate::settings::Settings;
use crate::test_support::app;
use daedyx_ui::theme;
use eframe::egui::accesskit::{Role, Toggled};
use eframe::egui::{Context, ViewportCommand};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

/// Einstellungen in einem hohen Fenster, damit alle Abschnitte ohne Bildlauf sichtbar und anklickbar sind.
pub(crate) fn settings_page() -> Harness<'static, crate::app::TasksApp> {
    let mut h = Harness::builder()
        .with_size((1280.0, 2400.0))
        .with_context_setup(|ctx| theme::install(ctx, daedyx_ui::tokens::AppAccent::Tasks))
        .build_ui_state(
            |ui, a: &mut crate::app::TasksApp| a.frame(ui),
            app(&Settings::default()),
        );
    h.run();
    h.get_by_role_and_label(Role::Tab, "Einstellungen").click();
    h.run();
    h
}

const KEEP: &str = "Im Infobereich weiterlaufen, wenn das Fenster geschlossen wird";

fn toggled(h: &Harness<'static, crate::app::TasksApp>, label: &str) -> Option<Toggled> {
    h.get_by_role_and_label(Role::CheckBox, label)
        .accesskit_node()
        .toggled()
}

#[test]
fn schalter_standard_infobereich_an_titel_aus() {
    let h = settings_page();
    assert_eq!(toggled(&h, KEEP), Some(Toggled::True));
    let titles = "Aufgabentitel in Benachrichtigungen zeigen";
    assert_eq!(toggled(&h, titles), Some(Toggled::False));
    let auto = "Mit Windows starten (im Hintergrund)";
    assert_eq!(toggled(&h, auto), Some(Toggled::False));
    assert!(h.state().bg.window.keep_running() && !h.state().bg.show_titles());
}

#[test]
fn schalter_aendern_hintergrund_und_autostart() {
    let mut h = settings_page();
    h.get_by_role_and_label(Role::CheckBox, KEEP).click();
    h.get_by_role_and_label(Role::CheckBox, "Aufgabentitel in Benachrichtigungen zeigen")
        .click();
    h.get_by_role_and_label(Role::CheckBox, "Mit Windows starten (im Hintergrund)")
        .click();
    h.run();
    let s = h.state();
    assert!(!s.bg.window.keep_running(), "Schließen beendet jetzt");
    assert!(s.bg.show_titles());
    assert_eq!(
        s.bg.autostart_state,
        Some(daedyx_background::autostart::State::On),
        "Ersatz-Autostart im Speicher"
    );
}

#[test]
fn testbenachrichtigung_meldet_erfolg() {
    let mut h = settings_page();
    h.get_by_role_and_label(Role::Button, "Testbenachrichtigung senden")
        .click();
    h.run();
    assert_eq!(h.state().bg.test_sent, Some(true));
    let _ = h.get_by_label("Gesendet. Sie erscheint unten rechts und im Info-Center.");
}

/// Schalter aus: Schließen wird ein „Beenden“; vorher gilt der Entwurf im Detailbereich.
#[test]
fn schalter_aus_schliessen_beendet_nach_dem_entwurf() {
    let mut a = app(&Settings::default());
    a.set_keep_running(false);
    // Eine Aufgabe ohne Fälligkeit: Datum und Uhrzeit des Entwurfs sind dann leer und unverändert.
    let list = a.tasks.lists.first().map(|l| l.id.clone()).unwrap();
    a.tasks.select(daedyx_tasks_ui::Sel::List(list));
    let t = a
        .tasks
        .items
        .iter()
        .find(|x| x.due.is_none())
        .cloned()
        .unwrap();
    a.tasks.draft.id = t.id.clone();
    a.tasks.draft.title = "Fenster putzen (erfunden)".into();
    a.tasks.draft.notes = t.notes.clone();
    assert_eq!(
        a.bg.window.on_close_requested(),
        vec![ViewportCommand::CancelClose]
    );
    a.logic(&Context::default());
    assert!(a.bg.window.quitting, "wirklich beendet");
    let now = a.tasks.items.iter().find(|x| x.id == t.id).unwrap();
    assert_eq!(now.title, "Fenster putzen (erfunden)");
    assert!(now.due.is_none(), "Fälligkeit unverändert");
}

#[test]
fn unbrauchbares_datum_haelt_die_app_beim_schliessen_offen() {
    let mut a = app(&Settings::default());
    a.set_keep_running(false);
    let t = a.tasks.items.first().cloned().unwrap();
    a.tasks.draft.id = t.id.clone();
    a.tasks.draft.title = t.title.clone();
    a.tasks.draft.notes = t.notes.clone();
    a.tasks.draft.date = "31.02.2026".into();
    let _ = a.bg.window.on_close_requested();
    a.logic(&Context::default());
    assert!(!a.bg.window.quitting && !a.bg.window.hidden);
    assert_eq!(a.tasks.error.as_deref(), Some("aufgaben.fehler.datum"));
}

#[test]
fn schalter_an_schliessen_versteckt_nur() {
    let mut a = app(&Settings::default());
    let cmds = a.bg.window.on_close_requested();
    assert!(cmds.contains(&ViewportCommand::Visible(false)));
    a.logic(&Context::default());
    assert!(a.bg.window.hidden && !a.bg.window.quitting);
}

#[test]
fn dunkelmodus_vorlage_regler_und_zuruecksetzen() {
    let mut h = settings_page();
    let _ = h.get_by_role_and_label(Role::Heading, "Dunkelmodus feinstellen");
    for name in ["Helligkeit", "Farbton", "Tönung"] {
        let _ = h.get_by_role_and_label(Role::Slider, name);
    }
    let group = h.get_by_role_and_label(Role::Group, "Vorlagen");
    group.get_by_role_and_label(Role::Button, "Grün").click();
    h.run();
    let tuning = h.state().dark_tuning.unwrap();
    assert_eq!(tuning.preset().map(|p| p.id), Some("gruen"));
    assert_eq!(
        theme::dark_tuning(&h.ctx),
        Some(tuning),
        "wirkt auf das Thema"
    );
    h.get_by_role_and_label(Role::Button, "Darstellung zurücksetzen")
        .click();
    h.run();
    assert_eq!(h.state().dark_tuning, None);
    assert_eq!(theme::dark_tuning(&h.ctx), None);
}

#[test]
fn ueber_zeigt_version_und_lizenz() {
    let h = settings_page();
    let _ = h.get_by_label("Version 0.2.0");
    let _ = h.get_by_role_and_label(Role::Heading, "Lizenzen");
    let _ = h.get_by_label("Daedyx Tasks steht unter der MIT-Lizenz.");
}

/// Review T4, N4-4: Die aktive Vorlage meldet sich als „ausgewählt“, nicht nur über die Farbe.
#[test]
fn aktive_dunkel_vorlage_ist_fuer_screenreader_ausgewaehlt() {
    let mut h = settings_page();
    let selected = |h: &Harness<'static, crate::app::TasksApp>, name: &str| {
        h.get_by_role_and_label(Role::Group, "Vorlagen")
            .get_by_role_and_label(Role::Button, name)
            .accesskit_node()
            .is_selected()
    };
    assert_eq!(selected(&h, "Grün"), Some(false), "ohne Feinstellung keine");
    h.get_by_role_and_label(Role::Group, "Vorlagen")
        .get_by_role_and_label(Role::Button, "Grün")
        .click();
    h.run();
    h.run();
    assert_eq!(selected(&h, "Grün"), Some(true));
    assert_eq!(selected(&h, "Lila"), Some(false));
}

/// Entscheid T4-3: Direkt am Schalter steht, was „aus“ kostet; beim Schließen gibt es keine Rückfrage.
#[test]
fn hinweis_am_infobereich_schalter() {
    let h = settings_page();
    let _ = h.get_by_label("Aus: Nach dem Schließen kommen keine Erinnerungen.");
    let _ = h.get_by_label(
        "An: Erinnerungen kommen auch ohne offenes Fenster, Schließen versteckt es nur.",
    );
}
