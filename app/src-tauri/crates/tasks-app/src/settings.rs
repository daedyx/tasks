//! Start-Einstellungen der Vorschau aus der Umgebung (zum Prüfen und für Bildschirmfotos): `DAEDYX_LANG=de|en|fr`,
//! `DAEDYX_THEME=dark|light`, `DAEDYX_SEITE=liste|board|einstellungen`,
//! `DAEDYX_STARTFEHLER=schluessel|zu_neu|platz|sicherung|anderes` (Startfehler-Fenster mit erfundenen Zahlen).
use crate::page::Page;
use daedyx_tasks_store::startup::StartupCode;
use daedyx_ui::i18n::Language;
use eframe::egui::Theme;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Settings {
    pub language: Language,
    /// `DAEDYX_LANG` war gesetzt: gewinnt über die gespeicherte Sprache (T5).
    pub language_from_env: bool,
    /// `None`: wie das System.
    pub dark: Option<bool>,
    pub page: Page,
    /// Startfehler-Fenster statt der App zeigen.
    pub startup_error: Option<StartupCode>,
}

fn var(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

/// Fehlercode zum Wort aus `DAEDYX_STARTFEHLER` (erfundene Zahlen beim fehlenden Platz).
pub fn code(word: &str) -> Option<StartupCode> {
    Some(match word {
        "schluessel" => StartupCode::WrongKey,
        "zu_neu" => StartupCode::TooNew,
        "platz" => StartupCode::NoSpace {
            needed_mb: 120,
            free_mb: 40,
        },
        "sicherung" => StartupCode::BackupFailed,
        "anderes" => StartupCode::Other,
        _ => return None,
    })
}

impl Settings {
    pub fn from_env() -> Settings {
        Settings {
            language: match var("DAEDYX_LANG").as_deref() {
                Some("en") => Language::En,
                Some("fr") => Language::Fr,
                _ => Language::De,
            },
            language_from_env: var("DAEDYX_LANG").is_some(),
            dark: match var("DAEDYX_THEME").as_deref() {
                Some("dark") => Some(true),
                Some("light") => Some(false),
                _ => None,
            },
            page: match var("DAEDYX_SEITE").as_deref() {
                Some("board") => Page::Board,
                Some("einstellungen") => Page::Settings,
                _ => Page::List,
            },
            startup_error: var("DAEDYX_STARTFEHLER").as_deref().and_then(code),
        }
    }
    /// Farbschema für egui: `None` folgt dem System.
    pub fn theme(&self) -> Option<Theme> {
        self.dark
            .map(|dark| if dark { Theme::Dark } else { Theme::Light })
    }
}
