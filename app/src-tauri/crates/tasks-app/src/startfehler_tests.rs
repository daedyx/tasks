//! Startfehler-Fenster je Fehlercode in drei Sprachen: Warnmeldung mit Titel, erklärender Satz ohne offene
//! Platzhalter, Knopf „Beenden“, der das Ende meldet.
use crate::settings::code;
use crate::startfehler::{message, window};
use crate::texts::texts;
use daedyx_tasks_store::startup::StartupCode;
use daedyx_ui::i18n::Language;
use daedyx_ui::startup::StartupError;
use daedyx_ui::theme;
use daedyx_ui::tokens::AppAccent;
use eframe::egui::accesskit::Role;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

const CODES: [&str; 5] = ["schluessel", "zu_neu", "platz", "sicherung", "anderes"];

#[test]
fn jeder_code_hat_einen_eigenen_satz_in_drei_sprachen() {
    for lang in Language::ALL {
        let t = texts(lang);
        let mut seen = Vec::new();
        for word in CODES {
            let text = message(code(word).unwrap(), &t);
            assert!(
                text.len() > 40 && !text.contains('{') && !text.contains("[["),
                "{lang:?} {word}: {text}"
            );
            assert!(!seen.contains(&text), "{lang:?} {word}: doppelt");
            seen.push(text);
        }
    }
    let t = texts(Language::De);
    let space = message(
        StartupCode::NoSpace {
            needed_mb: 120,
            free_mb: 40,
        },
        &t,
    );
    assert!(space.contains("120") && space.contains("40"));
    assert_eq!(code("unbekannt"), None);
}

#[test]
fn fenster_zeigt_titel_und_beenden_in_jeder_sprache() {
    for (lang, title, quit) in [
        (Language::De, "Daedyx Tasks kann nicht starten", "Beenden"),
        (Language::En, "Daedyx Tasks cannot start", "Quit"),
        (Language::Fr, "Daedyx Tasks ne peut pas démarrer", "Quitter"),
    ] {
        let w = window(StartupCode::TooNew, &texts(lang));
        let mut h = Harness::builder()
            .with_size((480.0, 320.0))
            .with_context_setup(|ctx| theme::install(ctx, AppAccent::Tasks))
            .build_ui_state(|ui, w: &mut StartupError| w.show(ui), w);
        h.run();
        let _ = h.get_by_role_and_label(Role::Alert, title);
        h.get_by_role_and_label(Role::Button, quit).click();
        h.run();
        assert!(h.state().quit_chosen(), "{lang:?}");
    }
}

/// Anrede wie Desk (Entscheid T2-3): Deutsch und Französisch duzen, Englisch bleibt neutral.
#[test]
fn startfehler_texte_duzen_wie_desk() {
    let words = |lang| {
        let t = texts(lang);
        CODES.map(|w| format!(" {} ", message(code(w).unwrap(), &t)))
    };
    for text in words(Language::De) {
        for sie in [" Sie ", " Ihre ", " Ihr ", " Ihnen "] {
            assert!(!text.contains(sie), "{sie}: {text}");
        }
    }
    for text in words(Language::Fr) {
        for vous in ["Veuillez", " vos ", " Vos ", " vous "] {
            assert!(!text.contains(vous), "{vous}: {text}");
        }
    }
    let de = words(Language::De).concat();
    assert!(de.contains(" dich ") && de.contains(" deine "));
}

/// Das Startfehler-Fenster nimmt das Farbschema der Einstellungen (`DAEDYX_THEME`), ohne Einstellung das System.
#[test]
fn startfehler_fenster_nimmt_das_farbschema_der_einstellungen() {
    use crate::settings::Settings;
    use daedyx_ui::startup::prepare;
    use eframe::egui::Theme;
    for (dark, want) in [
        (Some(true), Some(Theme::Dark)),
        (Some(false), Some(Theme::Light)),
        (None, None),
    ] {
        let settings = Settings {
            dark,
            ..Settings::default()
        };
        assert_eq!(settings.theme(), want);
    }
    let theme = Settings {
        dark: Some(true),
        ..Settings::default()
    }
    .theme();
    let h = Harness::builder()
        .with_size((480.0, 320.0))
        .with_context_setup(move |ctx| prepare(ctx, AppAccent::Tasks, theme))
        .build_ui_state(
            |ui, w: &mut StartupError| w.show(ui),
            window(StartupCode::Other, &texts(Language::De)),
        );
    assert_eq!(h.ctx.theme(), Theme::Dark);
}
