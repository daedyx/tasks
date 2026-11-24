//! Texte des Startfehler-Fensters. Das Verhalten (Code, Datei unverändert) prüft `daedyx-tasks-store`.
use crate::db::{DbError, NotEnoughSpace};
use crate::startup::message;
use daedyx_tasks_core::MigrateError;
use daedyx_tasks_store::startup::StartupError;

#[test]
fn every_startup_text_exists_in_the_language_file() {
    for key in ["title", "error.backup_failed", "error.no_space", "error.too_new", "error.wrong_key", "error.generic"] {
        let full = format!("start.{key}");
        assert_ne!(crate::startup_text::text(&full, &[]), full, "Text fehlt: {full}");
    }
}

#[test]
fn every_startup_code_has_a_plain_message() {
    let backup = message(&StartupError::BackupFailed(std::io::Error::other("x")));
    assert!(backup.contains("Sicherung"), "{backup}");
    let space = message(&StartupError::NoSpace(NotEnoughSpace { needed: 5 * 1024 * 1024, free: 1024 * 1024 }));
    assert!(space.contains("6 MB") && space.contains("1 MB"), "{space}");
    let too_new = message(&StartupError::Migrate(MigrateError::TooNew { found: 99, supported: 3 }));
    assert!(too_new.contains("neueren Version"), "{too_new}");
    assert!(!too_new.contains("99"), "keine technischen Zahlen im Dialog");
    let wrong = message(&StartupError::Open(DbError::WrongKey));
    assert!(wrong.contains("Schlüssel"), "{wrong}");
    let other = message(&StartupError::Open(DbError::BadKey));
    assert!(other.contains("Technische Angabe") && other.contains("ungültig"), "{other}");
}
