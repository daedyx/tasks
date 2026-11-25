//! Rahmen der App (T2): Start mit „Heute“, Rollen und Namen von Leiste, Kopf- und Statusleiste, Seitenwechsel,
//! Sprache umschalten, Leiste schmal und unten. Nur erfundene Daten.
use crate::page::Page;
use crate::settings::Settings;
use crate::shell::{Rail, rail};
use crate::test_support::{app, harness};
use daedyx_tasks_ui::Mode;
use eframe::egui::accesskit::Role;
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
    // Im Board hat die Aufgaben-Oberfläche einen eigenen Umschalter „Board“; gemeint ist der Reiter der Leiste.
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
fn leiste_wird_schmal_und_wandert_nach_unten() {
    assert_eq!(rail(1280.0), Rail::Wide);
    assert_eq!(rail(860.0), Rail::Wide);
    assert_eq!(rail(859.0), Rail::Narrow);
    assert_eq!(rail(640.0), Rail::Narrow);
    assert_eq!(rail(639.0), Rail::Bottom);
    for width in [700.0, 520.0] {
        let h = harness(app(&Settings::default()), width);
        let _ = h.get_by_role_and_label(Role::Tab, "Liste");
        let _ = h.get_by_role_and_label(Role::Tab, "Einstellungen");
    }
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
