//! Hilfen für Tests (eigene und die anderer Crates über das Merkmal `test-support`). Nie im Produkt-Bau.
use std::path::PathBuf;

/// Frischer, leerer Wegwerf-Ordner für einen Test: `<System-Temp>/daedyx-tasks-store-<pid>-<name>`. Nie der
/// Datenordner der App (SICHERHEIT 2.8); der Name enthält die Prozessnummer, damit parallele Läufe sich nicht stören.
#[allow(clippy::disallowed_methods, reason = "SICHERHEIT-AUSNAHMEN A-005: Test-Ordner im System-Temp")]
pub fn test_dir(name: &str) -> std::io::Result<PathBuf> {
    let p = std::env::temp_dir().join(format!("daedyx-tasks-store-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p)?;
    Ok(p)
}
