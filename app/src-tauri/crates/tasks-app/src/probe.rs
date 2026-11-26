//! Nur in der Vorschau übersetzt: Installationsprobe des Updates (plan-update-bibliothek 10, Muster M8). Die Probe
//! setzt `DAEDYX_UPDATE_PROBE=<Setup>` und `DAEDYX_UPDATE_PROBE_VON=<installierte Version>`; die laufende App startet
//! dann dieses Setup wie ein geprüftes Update (`/P /UPDATE /R`, versteckt mit `/ARGS --hidden`) und beendet sich. Den
//! Nachweis braucht die Probe nicht, sie prüft nur Start, Ende und Installer. Damit die neu gestartete App (gleiche
//! Umgebung) nicht noch einmal startet, gilt es nur, solange `app.json` neben dem Programm genau die Ausgangsversion
//! nennt. Die Produkt-Fassung enthält diese Datei nicht (Ausnahme in `docs/SICHERHEIT-AUSNAHMEN.md`).
use std::path::{Path, PathBuf};

pub const SETUP_VAR: &str = "DAEDYX_UPDATE_PROBE";
pub const FROM_VAR: &str = "DAEDYX_UPDATE_PROBE_VON";

/// Setup der Probe, wenn sie für das Programm in `exe_dir` gilt.
pub fn setup_for(exe_dir: &Path, setup: Option<String>, from: Option<String>) -> Option<PathBuf> {
    let (setup, from) = (setup.filter(|s| !s.is_empty())?, from?);
    let json = std::fs::read(exe_dir.join("app.json")).ok()?;
    let value: serde_json::Value = serde_json::from_slice(&json).ok()?;
    (value.get("version")?.as_str()? == from).then(|| PathBuf::from(setup))
}

/// Läuft die App in der Umgebung einer Probe? Dann startet keine echte Update-Suche (kein Netz), auch nicht in der
/// nach dem Update neu gestarteten App, für die [`setup`] nichts mehr liefert.
pub fn active() -> bool {
    std::env::var_os(SETUP_VAR).is_some_and(|v| !v.is_empty())
}

/// Setup der Probe aus der Umgebung und dem Ordner des laufenden Programms.
pub fn setup() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    setup_for(
        exe.parent()?,
        std::env::var(SETUP_VAR).ok(),
        std::env::var(FROM_VAR).ok(),
    )
}

#[cfg(test)]
mod tests {
    use super::setup_for;
    use crate::test_support::tmp;

    fn some(s: &str) -> Option<String> {
        Some(s.to_owned())
    }

    #[test]
    fn probe_gilt_nur_fuer_die_ausgangsversion_und_nie_doppelt() {
        let dir = tmp("u2-probe");
        let setup = some("C:/probe/setup.exe");
        // Ohne app.json neben dem Programm nie.
        assert_eq!(setup_for(&dir, setup.clone(), some("0.0.1")), None);
        std::fs::write(
            dir.join("app.json"),
            br#"{"version": "0.0.1", "exe": "x.exe"}"#,
        )
        .unwrap();
        assert!(setup_for(&dir, setup.clone(), some("0.0.1")).is_some());
        assert_eq!(setup_for(&dir, setup.clone(), None), None);
        assert_eq!(setup_for(&dir, some(""), some("0.0.1")), None);
        assert_eq!(setup_for(&dir, None, some("0.0.1")), None);
        // Nach dem Update nennt app.json die neue Version: Die neu gestartete App startet nichts mehr.
        std::fs::write(
            dir.join("app.json"),
            br#"{"version": "0.0.2", "exe": "x.exe"}"#,
        )
        .unwrap();
        assert_eq!(setup_for(&dir, setup, some("0.0.1")), None);
    }
}
