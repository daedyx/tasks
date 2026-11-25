//! Startfehler-Fenster der App: Text zum Fehlercode aus `daedyx-tasks-store` in der gewählten Sprache, angezeigt
//! mit `daedyx_ui::startup` (egui statt Meldungsfenster des Systems; die Tauri-Hülle behält ihres bis T7).
use daedyx_tasks_store::startup::StartupCode;
use daedyx_ui::i18n::Texts;
use daedyx_ui::startup::StartupError;

/// Erklärender Satz zum Code (ohne Pfad, ohne Inhalt der Datenbank).
pub fn message(code: StartupCode, texts: &Texts) -> String {
    match code {
        StartupCode::WrongKey => texts.t("start.fehler.falscher_schluessel"),
        StartupCode::TooNew => texts.t("start.fehler.zu_neu"),
        StartupCode::NoSpace { needed_mb, free_mb } => texts.tf(
            "start.fehler.kein_platz",
            &[
                ("noetig", &needed_mb.to_string()),
                ("frei", &free_mb.to_string()),
            ],
        ),
        StartupCode::BackupFailed => texts.t("start.fehler.sicherung"),
        StartupCode::Other => texts.t("start.fehler.anderes"),
    }
}

/// Inhalt des Fensters: Titel, Satz, „Beenden“.
pub fn window(code: StartupCode, texts: &Texts) -> StartupError {
    StartupError::new(
        &texts.t("start.titel"),
        &message(code, texts),
        &texts.t("start.beenden"),
    )
}
