//! Rahmen der App (T2): Start mit „Heute“, Rollen und Namen von Leiste, Kopf- und Statusleiste, Seitenwechsel,
//! Sprache umschalten, Leiste breit und unten, Umschalter der Aufgaben-Oberfläche aus. Nur erfundene Daten.
use crate::app::TasksApp;
use crate::page::Page;
use crate::settings::Settings;
use crate::shell::{Rail, rail};
use crate::test_support::{app, harness};
use daedyx_tasks_ui::Mode;
use eframe::egui::Rect;
use eframe::egui::accesskit::Role;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

#[test]
fn start_zeigt_heute_in_der_liste() {
    let h = harness(app(&Settings::default()), 1280.0);
    assert!(h.query_all_by_label("Heute").count() >= 1);
    assert_eq!(h.state().active(), Page::List);
}

#[test]
fn leiste_kopf_und_statusleiste_haben_rollen_und_namen() {
    let h = harness(app(&Settings::default()), 1280.0);
    let _ = h.get_by_role_and_label(Role::TabList, "Bereiche");
    let liste = h.get_by_role_and_label(Role::Tab, "Liste");
    assert_eq!(liste.accesskit_node().is_selected(), Some(true));
    let board = h.get_by_role_and_label(Role::Tab, "Board");
    assert_eq!(board.accesskit_node().is_selected(), Some(false));
    let _ = h.get_by_role_and_label(Role::Tab, "Einstellungen");
    let _ = h.get_by_role_and_label(Role::Banner, "Kopfzeile");
    let _ = h.get_by_role_and_label(Role::Status, "Statusleiste");
    let _ = h.get_by_label("Vorschau mit erfundenen Daten – es wird nichts gespeichert");
}

#[test]
fn board_reiter_oeffnet_die_erste_liste_als_board() {
    let mut h = harness(app(&Settings::default()), 1280.0);
    h.get_by_role_and_label(Role::Tab, "Board").click();
    h.run();
    assert_eq!(h.state().tasks.mode, Mode::Board);
    assert!(h.state().tasks.list_id().is_some());
    let rail = h.get_by_role_and_label(Role::TabList, "Bereiche");
    let board = rail.get_by_role_and_label(Role::Tab, "Board");
    assert_eq!(board.accesskit_node().is_selected(), Some(true));
}

#[test]
fn sprache_umschalten_in_den_einstellungen() {
    let mut h = harness(app(&Settings::default()), 1280.0);
    h.get_by_role_and_label(Role::Tab, "Einstellungen").click();
    h.run();
    let _ = h.get_by_label("Version 0.2.0");
    h.get_by_role_and_label(Role::Button, "English").click();
    h.run();
    let _ = h.get_by_role_and_label(Role::Tab, "List");
    let _ = h.get_by_role_and_label(Role::Tab, "Settings");
    h.get_by_role_and_label(Role::Button, "Français").click();
    h.run();
    let _ = h.get_by_role_and_label(Role::Tab, "Paramètres");
}

#[test]
fn leiste_bleibt_breit_und_wandert_unter_640_nach_unten() {
    assert_eq!(rail(1280.0), Rail::Wide);
    assert_eq!(rail(859.0), Rail::Wide);
    assert_eq!(rail(640.0), Rail::Wide);
    assert_eq!(rail(639.0), Rail::Bottom);
    // Form an den gezeichneten Rechtecken der Reiter (accesskit), nicht nur an `rail()` (Review T2, N-1). Die Liste
    // „Bereiche“ selbst hat kein Rechteck.
    for width in [1280.0, 700.0] {
        let h = harness(app(&Settings::default()), width);
        let [list, board, prefs] = tabs(&h);
        assert!(
            list.width() >= 150.0,
            "{width}: Reiter {list:?} nicht breit"
        );
        assert!(list.left() < 20.0, "{width}: Leiste {list:?} nicht links");
        assert!(
            list.top() < board.top() && board.top() < prefs.top(),
            "{width}: nicht untereinander"
        );
    }
    let h = harness(app(&Settings::default()), 520.0);
    let [list, board, prefs] = tabs(&h);
    assert!(list.top() > 400.0, "Leiste {list:?} nicht unten");
    assert!(
        list.right() <= board.left() && board.right() <= prefs.left(),
        "nicht nebeneinander"
    );
    assert!(
        (list.top() - prefs.top()).abs() < 1.0,
        "nicht in einer Zeile"
    );
    assert!(
        prefs.right() - list.left() > 3.0 * list.height(),
        "Leiste nicht quer"
    );
}

fn tabs(h: &Harness<'static, TasksApp>) -> [Rect; 3] {
    let rail = h.get_by_role_and_label(Role::TabList, "Bereiche");
    ["Liste", "Board", "Einstellungen"].map(|n| rail.get_by_role_and_label(Role::Tab, n).rect())
}

#[test]
fn umschalter_der_aufgaben_oberflaeche_ist_aus() {
    let mut h = harness(app(&Settings::default()), 1280.0);
    h.get_by_role_and_label(Role::Tab, "Board").click();
    h.run();
    assert!(!h.state().tasks.mode_switch);
    assert_eq!(
        h.query_all_by_role_and_label(Role::TabList, "Ansicht der Liste")
            .count(),
        0
    );
    assert_eq!(h.query_all_by_role_and_label(Role::Tab, "Board").count(), 1);
    let _ = h.get_by_label("Listen-Aktionen");
}

#[test]
fn startseite_aus_den_einstellungen() {
    let settings = Settings {
        page: Page::Board,
        ..Settings::default()
    };
    let h = harness(app(&settings), 1280.0);
    assert_eq!(h.state().active(), Page::Board);
}
